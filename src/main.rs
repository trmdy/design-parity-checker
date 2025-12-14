mod cli;

use std::io;
use std::path::Path;
use std::process::ExitCode;
use std::str::FromStr;

use cli::{Commands, OutputFormat, ResourceType};
use dpc_lib::types::{MetricScores, ResourceKind};
use dpc_lib::NormalizedView;
use dpc_lib::{
    calculate_combined_score, default_metrics, figma_to_normalized_view, image_to_normalized_view,
    parse_resource, run_metrics, url_to_normalized_view, CompareOutput, DpcError, DpcOutput,
    FigmaClient, FigmaRenderOptions, FindingSeverity, GenerateCodeOutput, ImageLoadOptions,
    MetricKind, ParsedResource, QualityFinding, QualityOutput, ResourceDescriptor, ScoreWeights,
    Summary, UrlToViewOptions, Viewport,
};

#[tokio::main]
async fn main() -> ExitCode {
    run().await
}

async fn run() -> ExitCode {
    let args = cli::parse();

    match args.command {
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
            keep_artifacts,
            ignore_selectors,
            ignore_regions,
            ..
        } => {
            if args.verbose {
                eprintln!("Parsing resources…");
            }
            let ref_res = match parse_resource(&r#ref, ref_type.map(resource_kind_from_cli)) {
                Ok(res) => res,
                Err(err) => return render_error(DpcError::Config(err.to_string()), format),
            };
            let impl_res = match parse_resource(&r#impl, impl_type.map(resource_kind_from_cli)) {
                Ok(res) => res,
                Err(err) => return render_error(DpcError::Config(err.to_string()), format),
            };
            let selected_metrics = match parse_metric_kinds(metrics.as_deref()) {
                Ok(k) => k,
                Err(err) => return render_error(DpcError::Config(err.to_string()), format),
            };
            let ignore_selectors = parse_ignore_selectors(ignore_selectors.as_deref());
            if let Some(path) = ignore_regions {
                return render_error(
                    DpcError::Config(format!(
                        "ignore-regions is not implemented yet (got: {})",
                        path.display()
                    )),
                    format,
                );
            }

            // Create temp directory for artifacts
            let artifacts_dir = std::env::temp_dir().join(format!("dpc-{}", std::process::id()));
            if let Err(err) = std::fs::create_dir_all(&artifacts_dir) {
                return render_error(DpcError::Io(err), format);
            }

            // Convert resources to NormalizedViews
            if args.verbose {
                eprintln!("Normalizing reference ({:?})…", ref_res.kind);
            }
            let ref_view_raw =
                match resource_to_normalized_view(&ref_res, &viewport, &artifacts_dir, "ref").await
                {
                    Ok(view) => view,
                    Err(err) => {
                        return render_error(
                            DpcError::Config(format!("Failed to process reference: {}", err)),
                            format,
                        )
                    }
                };

            if args.verbose {
                eprintln!("Normalizing implementation ({:?})…", impl_res.kind);
            }
            let impl_view_raw =
                match resource_to_normalized_view(&impl_res, &viewport, &artifacts_dir, "impl")
                    .await
                {
                    Ok(view) => view,
                    Err(err) => {
                        return render_error(
                            DpcError::Config(format!("Failed to process implementation: {}", err)),
                            format,
                        )
                    }
                };

            let ref_view = apply_dom_ignores(&ref_view_raw, &ignore_selectors);
            let impl_view = apply_dom_ignores(&impl_view_raw, &ignore_selectors);

            // Determine effective metrics based on input types
            // If no metrics specified and both inputs lack DOM data, use only image-compatible metrics
            let effective_metrics =
                if selected_metrics.is_empty() && ref_view.dom.is_none() && impl_view.dom.is_none()
                {
                    vec![MetricKind::Pixel, MetricKind::Color]
                } else {
                    selected_metrics
                };

            // Run metrics
            if args.verbose {
                eprintln!("Running metrics: {:?}", effective_metrics);
            }
            let all_metrics = default_metrics();
            let metrics_scores =
                match run_metrics(&all_metrics, &effective_metrics, &ref_view, &impl_view) {
                    Ok(scores) => scores,
                    Err(err) => {
                        return render_error(
                            DpcError::Config(format!("Failed to compute metrics: {}", err)),
                            format,
                        )
                    }
                };

            // Calculate combined score
            let weights = ScoreWeights::default();
            let similarity = calculate_combined_score(&metrics_scores, &weights);

            // Determine pass/fail
            let passed = similarity >= threshold as f32;

            // Generate summary
            let summary = generate_summary(&metrics_scores, similarity, threshold as f32);

            let body = DpcOutput::Compare(CompareOutput {
                version: env!("CARGO_PKG_VERSION").to_string(),
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
            });

            if let Err(err) = write_output(&body, format, output) {
                return render_error(DpcError::Config(err.to_string()), format);
            }

            // Cleanup artifacts unless --keep-artifacts is set
            if !keep_artifacts {
                let _ = std::fs::remove_dir_all(&artifacts_dir);
            } else if args.verbose {
                eprintln!("Artifacts saved to: {}", artifacts_dir.display());
            }

            exit_code_for_compare(passed)
        }
        Commands::GenerateCode {
            input,
            input_type,
            viewport,
            stack,
            output,
        } => {
            let viewport = Some(viewport);
            let input_res = match parse_resource(&input, input_type.map(resource_kind_from_cli)) {
                Ok(res) => res,
                Err(err) => {
                    return render_error(DpcError::Config(err.to_string()), OutputFormat::Json)
                }
            };
            let body = DpcOutput::GenerateCode(GenerateCodeOutput {
                version: env!("CARGO_PKG_VERSION").to_string(),
                input: ResourceDescriptor {
                    kind: input_res.kind,
                    value: input_res.value,
                },
                viewport,
                stack: Some(stack),
                output_path: output.clone(),
                code: None,
                summary: Some(Summary {
                    top_issues: vec!["generate-code is not implemented yet".to_string()],
                }),
            });
            if let Err(err) = write_output(&body, OutputFormat::Json, output) {
                return render_error(DpcError::Config(err.to_string()), OutputFormat::Json);
            }
            ExitCode::SUCCESS
        }
        Commands::Quality {
            input,
            input_type,
            viewport,
            format,
            output,
        } => {
            let input_res = match parse_resource(&input, input_type.map(resource_kind_from_cli)) {
                Ok(res) => res,
                Err(err) => return render_error(DpcError::Config(err.to_string()), format),
            };
            let body = DpcOutput::Quality(QualityOutput {
                version: env!("CARGO_PKG_VERSION").to_string(),
                input: ResourceDescriptor {
                    kind: input_res.kind,
                    value: input_res.value,
                },
                viewport,
                score: 0.0,
                findings: vec![QualityFinding {
                    severity: FindingSeverity::Info,
                    finding_type: "not_implemented".to_string(),
                    message: "quality mode not implemented yet".to_string(),
                }],
            });
            if let Err(err) = write_output(&body, format, output) {
                return render_error(DpcError::Config(err.to_string()), format);
            }
            ExitCode::SUCCESS
        }
    }
}

async fn resource_to_normalized_view(
    resource: &ParsedResource,
    viewport: &Viewport,
    artifacts_dir: &Path,
    prefix: &str,
) -> Result<NormalizedView, Box<dyn std::error::Error + Send + Sync>> {
    if matches!(resource.kind, ResourceKind::Url | ResourceKind::Figma) {
        if let Some(mock_path) = mock_render_image_path(prefix) {
            let screenshot_path = artifacts_dir.join(format!("{}_screenshot.png", prefix));
            let options = ImageLoadOptions {
                no_resize: false,
                target_width: Some(viewport.width),
                target_height: Some(viewport.height),
            };
            let view = image_to_normalized_view(
                mock_path.as_str(),
                screenshot_path.to_string_lossy().as_ref(),
                options,
            )
            .map_err(|e| format!("Mock rendering failed: {}", e))?;
            return Ok(view);
        }
    }

    match resource.kind {
        ResourceKind::Image => {
            let screenshot_path = artifacts_dir.join(format!("{}_screenshot.png", prefix));
            let options = ImageLoadOptions {
                no_resize: false,
                target_width: Some(viewport.width),
                target_height: Some(viewport.height),
            };
            let view = image_to_normalized_view(
                resource.value.as_str(),
                &screenshot_path.to_string_lossy(),
                options,
            )
            .map_err(|e| format!("Image loading failed: {}", e))?;
            Ok(view)
        }
        ResourceKind::Url => {
            let screenshot_path = artifacts_dir.join(format!("{}_screenshot.png", prefix));
            let mut options = UrlToViewOptions::default();
            options.viewport = *viewport;
            let view = url_to_normalized_view(resource.value.as_str(), &screenshot_path, options)
                .await
                .map_err(|e| format!("URL rendering failed: {}", e))?;
            Ok(view)
        }
        ResourceKind::Figma => {
            let figma_info = resource
                .figma_info
                .as_ref()
                .ok_or_else(|| DpcError::Config("Missing Figma file key".to_string()))?;
            let node_id = figma_info
                .node_id
                .clone()
                .ok_or_else(|| DpcError::Config("Figma node-id is required".to_string()))?;
            let token = std::env::var("FIGMA_TOKEN").map_err(|_| {
                DpcError::Config("FIGMA_TOKEN environment variable is required".to_string())
            })?;
            let client =
                FigmaClient::new(token).map_err(|e| format!("Figma client error: {}", e))?;
            let output_path = artifacts_dir.join(format!("{}_figma.png", prefix));
            let options = FigmaRenderOptions {
                file_key: figma_info.file_key.clone(),
                node_id,
                output_path,
                viewport: Some(*viewport),
                scale: 1.0,
            };
            let view = figma_to_normalized_view(&client, &options)
                .await
                .map_err(|e| format!("Figma rendering failed: {}", e))?;
            Ok(view)
        }
    }
}

fn mock_render_image_path(prefix: &str) -> Option<String> {
    let env_key = format!("DPC_MOCK_RENDER_{}", prefix.to_ascii_uppercase());
    if let Ok(path) = std::env::var(&env_key) {
        if !path.trim().is_empty() {
            return Some(path);
        }
    }

    if let Ok(dir) = std::env::var("DPC_MOCK_RENDERERS_DIR") {
        let candidate = std::path::Path::new(&dir).join(format!("{prefix}.png"));
        if candidate.exists() {
            return Some(candidate.to_string_lossy().into_owned());
        }
    }

    None
}

fn generate_summary(scores: &MetricScores, similarity: f32, threshold: f32) -> Summary {
    let mut top_issues = Vec::new();

    // Check each metric and generate human-readable issues
    if let Some(ref pixel) = scores.pixel {
        if pixel.score < 0.9 {
            let diff_pct = ((1.0 - pixel.score) * 100.0).round();
            top_issues.push(format!(
                "Pixel differences detected in ~{}% of the image",
                diff_pct
            ));
        }
        if !pixel.diff_regions.is_empty() {
            let major_regions = pixel
                .diff_regions
                .iter()
                .filter(|r| matches!(r.severity, dpc_lib::types::DiffSeverity::Major))
                .count();
            if major_regions > 0 {
                top_issues.push(format!(
                    "{} major visual difference region(s) found",
                    major_regions
                ));
            }
        }
    }

    if let Some(ref layout) = scores.layout {
        if layout.score < 0.9 {
            let missing = layout
                .diff_regions
                .iter()
                .filter(|r| matches!(r.kind, dpc_lib::types::LayoutDiffKind::MissingElement))
                .count();
            let extra = layout
                .diff_regions
                .iter()
                .filter(|r| matches!(r.kind, dpc_lib::types::LayoutDiffKind::ExtraElement))
                .count();
            let shifted = layout
                .diff_regions
                .iter()
                .filter(|r| matches!(r.kind, dpc_lib::types::LayoutDiffKind::PositionShift))
                .count();

            if missing > 0 {
                top_issues.push(format!(
                    "{} element(s) missing from implementation",
                    missing
                ));
            }
            if extra > 0 {
                top_issues.push(format!("{} extra element(s) in implementation", extra));
            }
            if shifted > 0 {
                top_issues.push(format!(
                    "{} element(s) shifted from expected position",
                    shifted
                ));
            }
        }
    }

    if let Some(ref typo) = scores.typography {
        if typo.score < 0.9 && !typo.diffs.is_empty() {
            let font_issues = typo
                .diffs
                .iter()
                .filter(|d| {
                    d.issues
                        .iter()
                        .any(|i| matches!(i, dpc_lib::types::TypographyIssue::FontFamilyMismatch))
                })
                .count();
            let size_issues = typo
                .diffs
                .iter()
                .filter(|d| {
                    d.issues
                        .iter()
                        .any(|i| matches!(i, dpc_lib::types::TypographyIssue::FontSizeDiff))
                })
                .count();

            if font_issues > 0 {
                top_issues.push(format!(
                    "{} element(s) have mismatched font families",
                    font_issues
                ));
            }
            if size_issues > 0 {
                top_issues.push(format!(
                    "{} element(s) have incorrect font sizes",
                    size_issues
                ));
            }
        }
    }

    if let Some(ref color) = scores.color {
        if color.score < 0.9 && !color.diffs.is_empty() {
            top_issues.push(format!(
                "{} color difference(s) detected in palette",
                color.diffs.len()
            ));
        }
    }

    if let Some(ref content) = scores.content {
        if content.score < 0.9 {
            if !content.missing_text.is_empty() {
                top_issues.push(format!(
                    "{} text element(s) missing from implementation",
                    content.missing_text.len()
                ));
            }
            if !content.extra_text.is_empty() {
                top_issues.push(format!(
                    "{} extra text element(s) in implementation",
                    content.extra_text.len()
                ));
            }
        }
    }

    // Add overall status
    if similarity >= threshold {
        top_issues.insert(
            0,
            format!(
                "Design parity check passed ({:.1}% similarity, threshold: {:.1}%)",
                similarity * 100.0,
                threshold * 100.0
            ),
        );
    } else {
        top_issues.insert(
            0,
            format!(
                "Design parity check failed ({:.1}% similarity, threshold: {:.1}%)",
                similarity * 100.0,
                threshold * 100.0
            ),
        );
    }

    Summary { top_issues }
}

fn resource_kind_from_cli(rt: ResourceType) -> ResourceKind {
    match rt {
        ResourceType::Url => ResourceKind::Url,
        ResourceType::Image => ResourceKind::Image,
        ResourceType::Figma => ResourceKind::Figma,
    }
}

fn parse_metric_kinds(
    kinds: Option<&[String]>,
) -> Result<Vec<MetricKind>, Box<dyn std::error::Error>> {
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

fn parse_ignore_selectors(raw: Option<&str>) -> Vec<String> {
    raw.map(|s| {
        s.split(',')
            .filter_map(|part| {
                let trimmed = part.trim().to_ascii_lowercase();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            })
            .collect()
    })
    .unwrap_or_default()
}

fn apply_dom_ignores(view: &NormalizedView, selectors: &[String]) -> NormalizedView {
    if selectors.is_empty() {
        return view.clone();
    }

    let mut filtered = view.clone();
    if let Some(dom) = &view.dom {
        let nodes = dom
            .nodes
            .iter()
            .filter(|n| !matches_any_selector(n, selectors))
            .cloned()
            .collect();
        let mut dom_filtered = dom.clone();
        dom_filtered.nodes = nodes;
        filtered.dom = Some(dom_filtered);
    }
    filtered
}

fn matches_any_selector(node: &dpc_lib::types::DomNode, selectors: &[String]) -> bool {
    selectors.iter().any(|sel| selector_matches(node, sel))
}

fn selector_matches(node: &dpc_lib::types::DomNode, selector: &str) -> bool {
    if let Some(id) = selector.strip_prefix('#') {
        let id = id.to_ascii_lowercase();
        let attr_id = node
            .attributes
            .get("id")
            .map(|v| v.to_ascii_lowercase())
            .unwrap_or_default();
        let node_id = node.id.to_ascii_lowercase();
        return attr_id == id || node_id == id;
    }

    if let Some(class) = selector.strip_prefix('.') {
        let class = class.to_ascii_lowercase();
        if let Some(attr) = node.attributes.get("class") {
            let has = attr
                .split_whitespace()
                .any(|c| c.eq_ignore_ascii_case(&class));
            if has {
                return true;
            }
        }
        return false;
    }

    node.tag.eq_ignore_ascii_case(selector)
}

fn write_output(
    body: &DpcOutput,
    format: OutputFormat,
    output: Option<std::path::PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = match format {
        OutputFormat::Json => serde_json::to_string(body)?,
        OutputFormat::Pretty => format_pretty(body),
    };

    if let Some(path) = output {
        std::fs::write(path, content)?;
    } else {
        println!("{}", content);
    }
    Ok(())
}

fn format_pretty(body: &DpcOutput) -> String {
    match body {
        DpcOutput::Compare(out) => {
            let status = if out.passed { "PASSED" } else { "FAILED" };
            let mut lines = vec![
                format!("Compare v{}", out.version),
                format!(
                    "ref:   {:?} {}",
                    out.ref_resource.kind, out.ref_resource.value
                ),
                format!(
                    "impl:  {:?} {}",
                    out.impl_resource.kind, out.impl_resource.value
                ),
                format!("viewport: {}x{}", out.viewport.width, out.viewport.height),
                format!(
                    "similarity: {:.2}% (threshold {:.2}%) -> {}",
                    out.similarity * 100.0,
                    out.threshold * 100.0,
                    status
                ),
            ];

            if let Some(summary) = &out.summary {
                if !summary.top_issues.is_empty() {
                    lines.push("Top issues:".to_string());
                    for issue in &summary.top_issues {
                        lines.push(format!("- {}", issue));
                    }
                }
            }

            if out.metrics.pixel.is_some()
                || out.metrics.layout.is_some()
                || out.metrics.typography.is_some()
                || out.metrics.color.is_some()
                || out.metrics.content.is_some()
            {
                lines.push("Metrics:".to_string());
                if let Some(m) = &out.metrics.pixel {
                    lines.push(format!("  pixel: {:.3}", m.score));
                }
                if let Some(m) = &out.metrics.layout {
                    lines.push(format!("  layout: {:.3}", m.score));
                }
                if let Some(m) = &out.metrics.typography {
                    lines.push(format!("  typography: {:.3}", m.score));
                }
                if let Some(m) = &out.metrics.color {
                    lines.push(format!("  color: {:.3}", m.score));
                }
                if let Some(m) = &out.metrics.content {
                    lines.push(format!("  content: {:.3}", m.score));
                }
            }

            lines.join("\n")
        }
        DpcOutput::GenerateCode(out) => {
            let mut lines = vec![
                format!("GenerateCode v{}", out.version),
                format!("input: {:?} {}", out.input.kind, out.input.value),
                format!(
                    "viewport: {}",
                    out.viewport
                        .map(|v| format!("{}x{}", v.width, v.height))
                        .unwrap_or_else(|| "unspecified".to_string())
                ),
            ];
            if let Some(stack) = &out.stack {
                lines.push(format!("stack: {}", stack));
            }
            if let Some(summary) = &out.summary {
                if !summary.top_issues.is_empty() {
                    lines.push("Notes:".to_string());
                    for issue in &summary.top_issues {
                        lines.push(format!("- {}", issue));
                    }
                }
            }
            lines.join("\n")
        }
        DpcOutput::Quality(out) => {
            let mut lines = vec![
                format!("Quality v{}", out.version),
                format!("input: {:?} {}", out.input.kind, out.input.value),
                format!("viewport: {}x{}", out.viewport.width, out.viewport.height),
                format!("score: {:.3}", out.score),
            ];
            if !out.findings.is_empty() {
                lines.push("Findings:".to_string());
                for f in &out.findings {
                    lines.push(format!(
                        "  [{:?}] {} - {}",
                        f.severity, f.finding_type, f.message
                    ));
                }
            }
            lines.join("\n")
        }
    }
}

fn render_error(err: DpcError, format: OutputFormat) -> ExitCode {
    let payload = err.to_payload();
    match format {
        OutputFormat::Json => {
            let json = serde_json::to_string(&payload).unwrap_or_else(|_| {
                "{\"category\":\"unknown\",\"message\":\"failed to serialize error\"}".to_string()
            });
            println!("{json}");
        }
        OutputFormat::Pretty => {
            eprintln!("{:?} error: {}", payload.category, payload.message);
            if let Some(hint) = payload.remediation {
                eprintln!("Hint: {}", hint);
            }
        }
    }
    // Reserve exit code 2 for fatal/errors; threshold failures use 1.
    ExitCode::from(2)
}

fn exit_code_for_compare(passed: bool) -> ExitCode {
    if passed {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dpc_lib::types::{BoundingBox, DomNode, DomSnapshot};
    use std::collections::HashMap;

    fn make_node(id: &str, tag: &str, class: Option<&str>) -> DomNode {
        let mut attrs = HashMap::new();
        if let Some(class) = class {
            attrs.insert("class".to_string(), class.to_string());
        }
        DomNode {
            id: id.to_string(),
            tag: tag.to_string(),
            children: vec![],
            parent: None,
            attributes: attrs,
            text: None,
            bounding_box: BoundingBox {
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
            },
            computed_style: None,
        }
    }

    fn view_with_dom(nodes: Vec<DomNode>) -> NormalizedView {
        NormalizedView {
            kind: ResourceKind::Url,
            screenshot_path: "dummy.png".into(),
            width: 100,
            height: 100,
            dom: Some(DomSnapshot {
                url: None,
                title: None,
                nodes,
            }),
            figma_tree: None,
            ocr_blocks: None,
        }
    }

    #[test]
    fn parse_ignore_selectors_normalizes_and_trims() {
        let parsed = parse_ignore_selectors(Some("  #Hero , .Ad ,p  ,, "));
        assert_eq!(parsed, vec!["#hero", ".ad", "p"]);
    }

    #[test]
    fn apply_dom_ignores_filters_on_id_class_and_tag() {
        let nodes = vec![
            make_node("hero", "div", Some("banner")),
            make_node("ad1", "div", Some("ad slot")),
            make_node("p1", "p", None),
        ];
        let view = view_with_dom(nodes);
        let selectors = vec!["#ad1".to_string(), ".banner".to_string(), "p".to_string()];
        let filtered = apply_dom_ignores(&view, &selectors);

        let kept: Vec<String> = filtered
            .dom
            .unwrap()
            .nodes
            .iter()
            .map(|n| n.id.clone())
            .collect();
        assert!(kept.is_empty(), "all nodes should be ignored");
    }

    #[test]
    fn exit_code_for_compare_maps_pass_fail() {
        assert_eq!(exit_code_for_compare(true), ExitCode::SUCCESS);
        assert_eq!(exit_code_for_compare(false), ExitCode::from(1));
    }

    #[test]
    fn render_error_always_returns_fatal_exit_code() {
        let code = render_error(
            DpcError::Config("boom".to_string()),
            OutputFormat::Json,
        );
        assert_eq!(code, ExitCode::from(2));
    }
}
