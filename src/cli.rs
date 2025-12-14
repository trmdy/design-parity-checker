use clap::{Parser, Subcommand, ValueEnum};
use dpc_lib::Viewport;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "dpc")]
#[command(
    version,
    about = "Design Parity Checker - Compare implementations against design references"
)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, global = true, help = "Enable verbose output")]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Compare a reference design against an implementation
    Compare {
        #[arg(long, help = "Reference resource (Figma URL, web URL, or local image)")]
        r#ref: String,

        #[arg(
            long,
            help = "Implementation resource (Figma URL, web URL, or local image)"
        )]
        r#impl: String,

        #[arg(long, value_enum, help = "Override type detection for reference")]
        ref_type: Option<ResourceType>,

        #[arg(long, value_enum, help = "Override type detection for implementation")]
        impl_type: Option<ResourceType>,

        #[arg(
            long,
            default_value = "1440x900",
            help = "Viewport dimensions (WIDTHxHEIGHT)"
        )]
        viewport: Viewport,

        #[arg(
            long,
            default_value = "0.95",
            help = "Similarity threshold for pass/fail"
        )]
        threshold: f64,

        #[arg(
            long,
            value_delimiter = ',',
            help = "Metrics to compute (pixel,layout,typography,color,content)"
        )]
        metrics: Option<Vec<String>>,

        #[arg(long, help = "CSS selectors to ignore in DOM comparisons")]
        ignore_selectors: Option<String>,

        #[arg(long, help = "Path to JSON file with regions to ignore")]
        ignore_regions: Option<PathBuf>,

        #[arg(long, value_enum, default_value = "json", help = "Output format")]
        format: OutputFormat,

        #[arg(long, short, help = "Output file path (stdout if omitted)")]
        output: Option<PathBuf>,

        #[arg(
            long,
            help = "Keep intermediate artifacts (screenshots, DOM snapshots)"
        )]
        keep_artifacts: bool,
    },

    /// Generate HTML/Tailwind code from a design input
    GenerateCode {
        #[arg(long, help = "Input resource (Figma URL, web URL, or local image)")]
        input: String,

        #[arg(long, value_enum, help = "Override type detection for input")]
        input_type: Option<ResourceType>,

        #[arg(long, default_value = "html+tailwind", help = "Output stack")]
        stack: String,

        #[arg(long, default_value = "1440x900", help = "Viewport dimensions")]
        viewport: Viewport,

        #[arg(long, short, help = "Output file path")]
        output: Option<PathBuf>,
    },

    /// Compute reference-free design quality score (experimental)
    Quality {
        #[arg(long, help = "Input resource (Figma URL, web URL, or local image)")]
        input: String,

        #[arg(long, value_enum, help = "Override type detection for input")]
        input_type: Option<ResourceType>,

        #[arg(long, default_value = "1440x900", help = "Viewport dimensions")]
        viewport: Viewport,

        #[arg(long, short, help = "Output file path")]
        output: Option<PathBuf>,

        #[arg(long, value_enum, default_value = "json", help = "Output format")]
        format: OutputFormat,
    },
}

#[derive(Clone, Copy, ValueEnum)]
pub enum ResourceType {
    Url,
    Image,
    Figma,
}

#[derive(Clone, Copy, ValueEnum, Default)]
pub enum OutputFormat {
    #[default]
    Json,
    Pretty,
}

pub fn parse() -> Cli {
    Cli::parse()
}

#[cfg(test)]
mod tests {
    use super::{Cli, Commands, OutputFormat, ResourceType};
    use clap::Parser;

    #[test]
    fn compare_command_uses_defaults() {
        let cli = Cli::parse_from([
            "dpc",
            "compare",
            "--ref",
            "https://example.com/design",
            "--impl",
            "https://example.com/build",
        ]);

        assert!(!cli.verbose);

        match cli.command {
            Commands::Compare {
                r#ref,
                r#impl,
                ref_type,
                impl_type,
                viewport,
                threshold,
                metrics,
                format,
                output,
                ignore_selectors,
                ignore_regions,
                keep_artifacts,
            } => {
                assert_eq!(r#ref, "https://example.com/design");
                assert_eq!(r#impl, "https://example.com/build");
                assert!(ref_type.is_none());
                assert!(impl_type.is_none());
                assert_eq!(viewport.width, 1440);
                assert_eq!(viewport.height, 900);
                assert!((threshold - 0.95).abs() < f64::EPSILON);
                assert!(metrics.is_none());
                assert!(matches!(format, OutputFormat::Json));
                assert!(output.is_none());
                assert!(ignore_selectors.is_none());
                assert!(ignore_regions.is_none());
                assert!(!keep_artifacts);
            }
            _ => panic!("expected compare command"),
        }
    }

    #[test]
    fn compare_command_respects_overrides() {
        let cli = Cli::parse_from([
            "dpc",
            "compare",
            "--ref",
            "ref.png",
            "--impl",
            "impl.png",
            "--ref-type",
            "image",
            "--impl-type",
            "figma",
            "--viewport",
            "1920x1080",
            "--threshold",
            "0.9",
            "--metrics",
            "pixel,layout",
            "--format",
            "pretty",
            "--output",
            "report.json",
            "--ignore-selectors",
            ".ads,.tracking",
            "--ignore-regions",
            "regions.json",
            "--keep-artifacts",
        ]);

        match cli.command {
            Commands::Compare {
                ref_type,
                impl_type,
                viewport,
                threshold,
                metrics,
                format,
                output,
                ignore_selectors,
                ignore_regions,
                keep_artifacts,
                ..
            } => {
                assert!(matches!(ref_type, Some(ResourceType::Image)));
                assert!(matches!(impl_type, Some(ResourceType::Figma)));
                assert_eq!(viewport.width, 1920);
                assert_eq!(viewport.height, 1080);
                assert!((threshold - 0.9).abs() < f64::EPSILON);
                assert_eq!(
                    metrics,
                    Some(vec![String::from("pixel"), String::from("layout"),])
                );
                assert!(matches!(format, OutputFormat::Pretty));
                assert_eq!(output.as_deref(), Some(std::path::Path::new("report.json")));
                assert_eq!(ignore_selectors.as_deref(), Some(".ads,.tracking"));
                assert_eq!(
                    ignore_regions.as_deref(),
                    Some(std::path::Path::new("regions.json"))
                );
                assert!(keep_artifacts);
            }
            _ => panic!("expected compare command with overrides"),
        }
    }

    #[test]
    fn quality_command_sets_verbose() {
        let cli = Cli::parse_from([
            "dpc",
            "--verbose",
            "quality",
            "--input",
            "https://example.com/page",
        ]);

        assert!(cli.verbose);

        match cli.command {
            Commands::Quality {
                input,
                format,
                output,
                input_type,
                viewport,
            } => {
                assert_eq!(input, "https://example.com/page");
                assert!(matches!(format, OutputFormat::Json));
                assert!(output.is_none());
                assert!(input_type.is_none());
                assert_eq!(viewport.width, 1440);
                assert_eq!(viewport.height, 900);
            }
            _ => panic!("expected quality command"),
        }
    }
}
