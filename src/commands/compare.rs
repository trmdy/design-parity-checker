use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use dpc_lib::output::DPC_OUTPUT_VERSION;
use dpc_lib::types::ResourceKind;
use dpc_lib::{
    calculate_combined_score, parse_resource, run_metrics, ColorPaletteMetric, CompareOutput,
    ContentSimilarity, DpcError, DpcOutput, LayoutSimilarity, Metric, MetricKind, PixelSimilarity,
    ResourceDescriptor, SemanticAnalyzer, TypographySimilarity, Viewport,
};

use crate::cli::OutputFormat;
use crate::formatting::{exit_code_for_compare, render_error, write_output};
use crate::pipeline::{
    apply_dom_ignores, apply_ignore_regions, generate_summary, load_ignore_regions,
    parse_ignore_selectors, persist_compare_artifacts, resolve_artifacts_dir,
    resource_to_normalized_view,
};
use crate::settings::{
    format_effective_config, load_config, log_effective_config, resolve_compare_settings,
    CompareFlagSources,
};

/// Run the compare command.
#[allow(clippy::too_many_arguments)]
pub async fn run_compare(
    raw_args: &[String],
    config_path: Option<PathBuf>,
    verbose: bool,
    r#ref: String,
    r#impl: String,
    ref_type: Option<crate::cli::ResourceType>,
    impl_type: Option<crate::cli::ResourceType>,
    viewport: Viewport,
    threshold: f64,
    metrics: Option<Vec<String>>,
    format: OutputFormat,
    output: Option<PathBuf>,
    keep_artifacts: bool,
    ignore_selectors: Option<String>,
    ignore_regions: Option<PathBuf>,
    artifacts_dir: Option<PathBuf>,
    nav_timeout: u64,
    network_idle_timeout: u64,
    process_timeout: u64,
    pixel_align: Option<bool>,
    pixel_align_max_shift: Option<u32>,
    pixel_align_downscale: Option<u32>,
    semantic_analysis: bool,
    context: Option<String>,
) -> ExitCode {
    let config = match load_config(config_path.as_deref()) {
        Ok(cfg) => cfg,
        Err(err) => return render_error(err, format, output.clone()),
    };
    let config_source = config_path.as_deref();
    let flag_sources = CompareFlagSources::from_args(raw_args);
    let resolved = resolve_compare_settings(
        viewport,
        threshold,
        nav_timeout,
        network_idle_timeout,
        process_timeout,
        pixel_align,
        pixel_align_max_shift,
        pixel_align_downscale,
        &config,
        &flag_sources,
    );
    let viewport = resolved.viewport;
    let threshold = resolved.threshold;
    let nav_timeout = resolved.nav_timeout;
    let network_idle_timeout = resolved.network_idle_timeout;
    let process_timeout = resolved.process_timeout;
    let score_weights = resolved.weights;
    let pixel_alignment = resolved.pixel_alignment;

    if verbose {
        log_effective_config(
            config_path.as_deref(),
            &viewport,
            threshold,
            &score_weights,
            nav_timeout,
            network_idle_timeout,
            process_timeout,
            &pixel_alignment,
        );
    }
    if verbose {
        eprintln!(
            "{}",
            format_effective_config(
                &viewport,
                threshold,
                nav_timeout,
                network_idle_timeout,
                process_timeout,
                &score_weights,
                &pixel_alignment,
                config_source
            )
        );
        eprintln!("Parsing resources\u{2026}");
    }

    let ref_res = match parse_resource(&r#ref, ref_type.map(resource_kind_from_cli)) {
        Ok(res) => res,
        Err(err) => return render_error(DpcError::Config(err.to_string()), format, output.clone()),
    };
    let impl_res = match parse_resource(&r#impl, impl_type.map(resource_kind_from_cli)) {
        Ok(res) => res,
        Err(err) => return render_error(DpcError::Config(err.to_string()), format, output.clone()),
    };

    let selected_metrics = match parse_metric_kinds(metrics.as_deref()) {
        Ok(k) => k,
        Err(err) => return render_error(DpcError::Config(err.to_string()), format, output.clone()),
    };
    let ignore_selectors = parse_ignore_selectors(ignore_selectors.as_deref());
    let ignore_regions = match ignore_regions {
        Some(path) => match load_ignore_regions(&path) {
            Ok(regions) => regions,
            Err(err) => return render_error(err, format, output.clone()),
        },
        None => Vec::new(),
    };

    // Create temp directory for artifacts
    let (artifacts_dir, artifacts_from_cli) = resolve_artifacts_dir(artifacts_dir.as_deref());
    if let Err(err) = std::fs::create_dir_all(&artifacts_dir) {
        return render_error(DpcError::Io(err), format, output.clone());
    }
    let should_keep_artifacts = keep_artifacts || artifacts_from_cli;
    let progress_logger: Option<Arc<dyn Fn(&str) + Send + Sync>> = if verbose {
        Some(Arc::new(|msg: &str| eprintln!("{msg}")))
    } else {
        None
    };

    // Convert resources to NormalizedViews
    if verbose {
        eprintln!("Normalizing reference ({:?})\u{2026}", ref_res.kind);
    }
    let ref_view_raw = match resource_to_normalized_view(
        &ref_res,
        &viewport,
        &artifacts_dir,
        "ref",
        progress_logger.clone(),
        nav_timeout,
        network_idle_timeout,
        process_timeout,
    )
    .await
    {
        Ok(view) => view,
        Err(err) => {
            return render_error(
                DpcError::Config(format!("Failed to process reference: {}", err)),
                format,
                output.clone(),
            )
        }
    };

    if verbose {
        eprintln!("Normalizing implementation ({:?})\u{2026}", impl_res.kind);
    }
    let impl_view_raw = match resource_to_normalized_view(
        &impl_res,
        &viewport,
        &artifacts_dir,
        "impl",
        progress_logger.clone(),
        nav_timeout,
        network_idle_timeout,
        process_timeout,
    )
    .await
    {
        Ok(view) => view,
        Err(err) => {
            return render_error(
                DpcError::Config(format!("Failed to process implementation: {}", err)),
                format,
                output.clone(),
            )
        }
    };

    let ref_view = apply_dom_ignores(&ref_view_raw, &ignore_selectors);
    let impl_view = apply_dom_ignores(&impl_view_raw, &ignore_selectors);

    let ref_view = if ignore_regions.is_empty() {
        ref_view
    } else {
        match apply_ignore_regions(&ref_view, &ignore_regions, &artifacts_dir, "ref") {
            Ok(view) => view,
            Err(err) => return render_error(err, format, output.clone()),
        }
    };
    let impl_view = if ignore_regions.is_empty() {
        impl_view
    } else {
        match apply_ignore_regions(&impl_view, &ignore_regions, &artifacts_dir, "impl") {
            Ok(view) => view,
            Err(err) => return render_error(err, format, output.clone()),
        }
    };

    // Determine effective metrics based on input types
    let effective_metrics =
        if selected_metrics.is_empty() && ref_view.dom.is_none() && impl_view.dom.is_none() {
            vec![MetricKind::Pixel, MetricKind::Color]
        } else {
            selected_metrics
        };

    // Run metrics
    if verbose {
        eprintln!("Running metrics: {:?}", effective_metrics);
    }
    let mut pixel_metric = PixelSimilarity::default();
    pixel_metric.alignment = pixel_alignment;
    let all_metrics: Vec<Box<dyn Metric>> = vec![
        Box::new(pixel_metric),
        Box::new(LayoutSimilarity::default()),
        Box::new(TypographySimilarity::default()),
        Box::new(ColorPaletteMetric::default()),
        Box::new(ContentSimilarity::default()),
    ];
    let mut metrics_scores =
        match run_metrics(&all_metrics, &effective_metrics, &ref_view, &impl_view) {
            Ok(scores) => scores,
            Err(err) => {
                return render_error(
                    DpcError::Config(format!("Failed to compute metrics: {}", err)),
                    format,
                    output.clone(),
                )
            }
        };

    // Run semantic analysis if enabled and we have pixel diff regions
    if semantic_analysis {
        if let Some(ref mut pixel_metric) = metrics_scores.pixel {
            if !pixel_metric.diff_regions.is_empty() {
                if verbose {
                    eprintln!(
                        "Running semantic analysis on {} diff regions...",
                        pixel_metric.diff_regions.len()
                    );
                }

                if let Some(analyzer) = SemanticAnalyzer::from_config(&config.semantic) {
                    // Use image-aware clustering to separate different UI components
                    match analyzer
                        .analyze_diff_regions(
                            &ref_view.screenshot_path,
                            &impl_view.screenshot_path,
                            &pixel_metric.diff_regions,
                            context.as_deref(),
                        )
                        .await
                    {
                        Ok(semantic_diffs) => {
                            if verbose {
                                eprintln!(
                                    "Semantic analysis found {} diff types",
                                    semantic_diffs.len()
                                );
                                for diff in &semantic_diffs {
                                    eprintln!("  - {}: {}", diff.diff_type, diff.description);
                                }
                            }
                            // Convert to the types expected by PixelMetric
                            let typed_diffs: Vec<dpc_lib::types::SemanticDiff> = semantic_diffs
                                .into_iter()
                                .map(|d| dpc_lib::types::SemanticDiff {
                                    x: d.x,
                                    y: d.y,
                                    width: d.width,
                                    height: d.height,
                                    severity: d.severity,
                                    diff_type: match d.diff_type {
                                        dpc_lib::SemanticDiffType::TextContent => {
                                            dpc_lib::types::SemanticDiffType::TextContent
                                        }
                                        dpc_lib::SemanticDiffType::TextReflow => {
                                            dpc_lib::types::SemanticDiffType::TextReflow
                                        }
                                        dpc_lib::SemanticDiffType::Typography => {
                                            dpc_lib::types::SemanticDiffType::Typography
                                        }
                                        dpc_lib::SemanticDiffType::Layout => {
                                            dpc_lib::types::SemanticDiffType::Layout
                                        }
                                        dpc_lib::SemanticDiffType::Color => {
                                            dpc_lib::types::SemanticDiffType::Color
                                        }
                                        dpc_lib::SemanticDiffType::MissingElement => {
                                            dpc_lib::types::SemanticDiffType::MissingElement
                                        }
                                        dpc_lib::SemanticDiffType::ExtraElement => {
                                            dpc_lib::types::SemanticDiffType::ExtraElement
                                        }
                                        dpc_lib::SemanticDiffType::Spacing => {
                                            dpc_lib::types::SemanticDiffType::Spacing
                                        }
                                        dpc_lib::SemanticDiffType::ImageChange => {
                                            dpc_lib::types::SemanticDiffType::ImageChange
                                        }
                                        dpc_lib::SemanticDiffType::Decoration => {
                                            dpc_lib::types::SemanticDiffType::Decoration
                                        }
                                        dpc_lib::SemanticDiffType::Other => {
                                            dpc_lib::types::SemanticDiffType::Other
                                        }
                                    },
                                    description: d.description,
                                    confidence: d.confidence,
                                })
                                .collect();
                            pixel_metric.semantic_diffs = Some(typed_diffs);
                        }
                        Err(e) => {
                            eprintln!("Warning: Semantic analysis failed: {}", e);
                        }
                    }
                } else {
                    eprintln!(
                        "Warning: --semantic-analysis requires an API key. Set DPC_VISION_API_KEY, OPENAI_API_KEY, or add [semantic] api_key to config"
                    );
                }
            }
        }
    }

    // Calculate combined score
    let similarity = calculate_combined_score(&metrics_scores, &score_weights);

    // Determine pass/fail
    let passed = similarity >= threshold as f32;

    // Generate summary
    let summary = generate_summary(&metrics_scores, similarity, threshold as f32);

    let artifacts = match persist_compare_artifacts(
        &artifacts_dir,
        &ref_view,
        &impl_view,
        should_keep_artifacts,
        pixel_alignment,
    ) {
        Ok(paths) => Some(paths),
        Err(err) => return render_error(err, format, output.clone()),
    };

    if let Some(paths) = &artifacts {
        let should_log = matches!(format, OutputFormat::Json | OutputFormat::Pretty)
            || verbose
            || paths.kept
            || artifacts_from_cli;
        if should_log {
            eprintln!(
                "Artifacts directory: {} (kept: {}; use --keep-artifacts or --artifacts-dir to retain)",
                paths.directory.display(),
                paths.kept
            );
            if verbose {
                if let Some(path) = &paths.ref_screenshot {
                    eprintln!("  ref screenshot: {}", path.display());
                }
                if let Some(path) = &paths.impl_screenshot {
                    eprintln!("  impl screenshot: {}", path.display());
                }
                if let Some(path) = &paths.ref_dom_snapshot {
                    eprintln!("  ref DOM: {}", path.display());
                }
                if let Some(path) = &paths.impl_dom_snapshot {
                    eprintln!("  impl DOM: {}", path.display());
                }
                if let Some(path) = &paths.ref_figma_snapshot {
                    eprintln!("  ref figma tree: {}", path.display());
                }
                if let Some(path) = &paths.impl_figma_snapshot {
                    eprintln!("  impl figma tree: {}", path.display());
                }
                if let Some(path) = &paths.diff_image {
                    eprintln!("  pixel diff: {}", path.display());
                } else {
                    eprintln!("  pixel diff: not generated");
                }
                if paths.diff_image.is_some()
                    || paths.ref_dom_snapshot.is_some()
                    || paths.impl_dom_snapshot.is_some()
                    || paths.ref_figma_snapshot.is_some()
                    || paths.impl_figma_snapshot.is_some()
                {
                    eprintln!(
                        "Hint: open diff_heatmap.png and DOM/Figma snapshots under {}",
                        paths.directory.display()
                    );
                }
                if !paths.kept {
                    eprintln!(
                        "Artifacts will be cleaned up; pass --keep-artifacts or --artifacts-dir to retain."
                    );
                }
            } else if paths.kept {
                if paths.diff_image.is_some()
                    || paths.ref_dom_snapshot.is_some()
                    || paths.impl_dom_snapshot.is_some()
                {
                    eprintln!(
                        "Hint: view diff heatmap or DOM snapshots in {}",
                        paths.directory.display()
                    );
                }
            }
        }
    }

    let body = DpcOutput::Compare(CompareOutput {
        version: DPC_OUTPUT_VERSION.to_string(),
        ref_resource: ResourceDescriptor {
            kind: ref_res.kind,
            value: ref_res.value,
        },
        impl_resource: ResourceDescriptor {
            kind: impl_res.kind,
            value: impl_res.value,
        },
        viewport,
        similarity,
        threshold: threshold as f32,
        passed,
        metrics: metrics_scores,
        summary: Some(summary),
        artifacts,
    });

    if let Err(err) = write_output(&body, format, output.clone()) {
        return render_error(DpcError::Config(err.to_string()), format, output);
    }

    // Cleanup artifacts unless --keep-artifacts is set
    if !should_keep_artifacts {
        let _ = std::fs::remove_dir_all(&artifacts_dir);
    }

    exit_code_for_compare(passed)
}

fn resource_kind_from_cli(rt: crate::cli::ResourceType) -> ResourceKind {
    match rt {
        crate::cli::ResourceType::Url => ResourceKind::Url,
        crate::cli::ResourceType::Image => ResourceKind::Image,
        crate::cli::ResourceType::Figma => ResourceKind::Figma,
    }
}

fn parse_metric_kinds(
    kinds: Option<&[String]>,
) -> Result<Vec<MetricKind>, Box<dyn std::error::Error>> {
    use std::io;
    use std::str::FromStr;

    let mut parsed = Vec::new();
    if let Some(items) = kinds {
        for item in items {
            let kind = MetricKind::from_str(item).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("invalid metric kind '{}': {}", item, e),
                )
            })?;
            parsed.push(kind);
        }
    }
    Ok(parsed)
}
