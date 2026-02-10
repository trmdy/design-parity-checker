use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::sync::Arc;

use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use dpc_lib::output::DPC_OUTPUT_VERSION;
use dpc_lib::types::ResourceKind;
use dpc_lib::{
    parse_resource, DpcError, DpcOutput, GenerateCodeOutput, ResourceDescriptor, Summary, Viewport,
};
use serde::{Deserialize, Serialize};

use crate::cli::OutputFormat;
use crate::formatting::{render_error, write_output};
use crate::pipeline::{resolve_artifacts_dir, resource_to_normalized_view};
use crate::progress::ProgressCallback;
use crate::settings::{flag_present, load_config};

/// Run the generate-code command.
#[allow(clippy::too_many_arguments)]
pub async fn run_generate_code(
    raw_args: &[String],
    config_path: Option<PathBuf>,
    verbose: bool,
    input: String,
    input_type: Option<crate::cli::ResourceType>,
    viewport: Viewport,
    stack: String,
    output: Option<PathBuf>,
    format: OutputFormat,
) -> ExitCode {
    let config = match load_config(config_path.as_deref()) {
        Ok(cfg) => cfg,
        Err(err) => return render_error(err, format, None),
    };
    let viewport = if flag_present(raw_args, "--viewport") {
        viewport
    } else {
        config.viewport
    };
    let timeouts = config.timeouts;
    let nav_timeout = timeouts.navigation.as_secs();
    let network_idle_timeout = timeouts.network_idle.as_secs();
    let process_timeout = timeouts.process.as_secs();

    let normalized_stack = match normalize_stack(&stack) {
        Ok(s) => s,
        Err(err) => return render_error(err, format, None),
    };

    if verbose {
        eprintln!("Parsing input resource…");
    }
    let input_res = match parse_resource(&input, input_type.map(resource_kind_from_cli)) {
        Ok(res) => res,
        Err(err) => return render_error(DpcError::Config(err.to_string()), format, None),
    };

    match mock_codegen_from_env() {
        Ok(Some(codegen)) => {
            if let Some(path) = &output {
                if let Err(err) = std::fs::write(path, codegen.code.as_bytes()) {
                    return render_error(DpcError::Io(err), format, None);
                }
            }
            let summary = if codegen.warnings.is_empty() {
                None
            } else {
                Some(Summary {
                    top_issues: codegen.warnings,
                })
            };
            let body = DpcOutput::GenerateCode(GenerateCodeOutput {
                version: DPC_OUTPUT_VERSION.to_string(),
                input: ResourceDescriptor {
                    kind: input_res.kind,
                    value: input_res.value,
                },
                viewport: Some(viewport),
                stack: Some(normalized_stack.clone()),
                output_path: output.clone(),
                code: Some(codegen.code),
                summary,
            });
            if let Err(err) = write_output(&body, format, None) {
                return render_error(DpcError::Config(err.to_string()), format, None);
            }
            return ExitCode::SUCCESS;
        }
        Ok(None) => {}
        Err(err) => return render_error(err, format, None),
    }

    let (artifacts_dir, _from_cli) = resolve_artifacts_dir(None);
    if let Err(err) = std::fs::create_dir_all(&artifacts_dir) {
        return render_error(DpcError::Io(err), format, None);
    }
    if verbose {
        eprintln!(
            "Normalizing input ({:?})… (artifacts: {})",
            input_res.kind,
            artifacts_dir.display()
        );
    }
    let progress_logger: Option<ProgressCallback> = if verbose {
        Some(Arc::new(|msg: &str| eprintln!("{msg}")))
    } else {
        None
    };
    let view = match resource_to_normalized_view(
        &input_res,
        &viewport,
        &artifacts_dir,
        "input",
        progress_logger,
        nav_timeout,
        network_idle_timeout,
        process_timeout,
    )
    .await
    {
        Ok(view) => view,
        Err(err) => {
            return render_error(
                DpcError::Config(format!("Failed to process input: {err}")),
                format,
                None,
            )
        }
    };

    if verbose {
        eprintln!(
            "Captured screenshot at {} ({}x{})",
            view.screenshot_path.display(),
            view.width,
            view.height
        );
    }

    let codegen = match generate_code(
        &view.screenshot_path,
        &normalized_stack,
        Some(viewport),
        verbose,
    )
    .await
    {
        Ok(res) => res,
        Err(err) => return render_error(err, format, None),
    };

    if let Some(path) = &output {
        if verbose {
            eprintln!("Writing generated code to {}", path.display());
        }
        if let Err(err) = std::fs::write(path, codegen.code.as_bytes()) {
            return render_error(DpcError::Io(err), format, None);
        }
    }

    let summary = if codegen.warnings.is_empty() {
        None
    } else {
        Some(Summary {
            top_issues: codegen.warnings.clone(),
        })
    };

    let body = DpcOutput::GenerateCode(GenerateCodeOutput {
        version: DPC_OUTPUT_VERSION.to_string(),
        input: ResourceDescriptor {
            kind: input_res.kind,
            value: input_res.value,
        },
        viewport: Some(viewport),
        stack: Some(normalized_stack),
        output_path: output.clone(),
        code: Some(codegen.code),
        summary,
    });
    if let Err(err) = write_output(&body, format, None) {
        return render_error(DpcError::Config(err.to_string()), format, None);
    }
    ExitCode::SUCCESS
}

fn resource_kind_from_cli(rt: crate::cli::ResourceType) -> ResourceKind {
    match rt {
        crate::cli::ResourceType::Url => ResourceKind::Url,
        crate::cli::ResourceType::Image => ResourceKind::Image,
        crate::cli::ResourceType::Figma => ResourceKind::Figma,
    }
}

fn normalize_stack(stack: &str) -> Result<String, DpcError> {
    let normalized = stack.trim().to_ascii_lowercase();
    if normalized == "html+tailwind" {
        return Ok(normalized);
    }

    Err(DpcError::Config(format!(
        "Unsupported stack '{}'; supported: html+tailwind",
        stack
    )))
}

struct CodegenResult {
    code: String,
    warnings: Vec<String>,
}

fn mock_codegen_from_env() -> Result<Option<CodegenResult>, DpcError> {
    if let Ok(mock) = std::env::var("DPC_MOCK_CODE") {
        if !mock.trim().is_empty() {
            return Ok(Some(CodegenResult {
                code: mock,
                warnings: vec!["Using DPC_MOCK_CODE; external codegen not invoked.".to_string()],
            }));
        }
    }

    if let Ok(path) = std::env::var("DPC_MOCK_CODE_PATH") {
        if !path.trim().is_empty() {
            let code = std::fs::read_to_string(&path)
                .map_err(|e| DpcError::Config(format!("Failed to read mock code file: {e}")))?;
            return Ok(Some(CodegenResult {
                code,
                warnings: vec![format!(
                    "Using mock code from {}; external codegen not invoked.",
                    path
                )],
            }));
        }
    }

    Ok(None)
}

async fn generate_code(
    screenshot_path: &Path,
    stack: &str,
    viewport: Option<Viewport>,
    verbose: bool,
) -> Result<CodegenResult, DpcError> {
    if let Ok(cmd) = std::env::var("DPC_CODEGEN_CMD") {
        if !cmd.trim().is_empty() {
            if verbose {
                eprintln!("Invoking codegen command: {}", cmd);
            }
            return run_codegen_command(&cmd, screenshot_path, stack);
        }
    }

    if let Ok(url) = std::env::var("DPC_CODEGEN_URL") {
        if !url.trim().is_empty() {
            if verbose {
                eprintln!("Calling codegen endpoint: {}", url);
            }
            return call_http_codegen(&url, screenshot_path, stack, viewport).await;
        }
    }

    Err(DpcError::Config(
        "No codegen backend configured; set DPC_CODEGEN_URL, DPC_CODEGEN_CMD, or DPC_MOCK_CODE."
            .to_string(),
    ))
}

fn run_codegen_command(
    cmd: &str,
    screenshot_path: &Path,
    stack: &str,
) -> Result<CodegenResult, DpcError> {
    let mut command = Command::new(cmd);

    if let Ok(args) = std::env::var("DPC_CODEGEN_ARGS") {
        for arg in args.split_whitespace() {
            command.arg(arg);
        }
    }

    command.arg(screenshot_path);
    command.arg(stack);

    let output = command
        .output()
        .map_err(|e| DpcError::Config(format!("Failed to run codegen command '{cmd}': {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(DpcError::Config(format!(
            "Codegen command exited with {}: {}",
            output.status,
            stderr.trim()
        )));
    }

    let stdout = String::from_utf8(output.stdout).unwrap_or_default();
    if stdout.trim().is_empty() {
        return Err(DpcError::Config(
            "Codegen command produced no output".to_string(),
        ));
    }

    let mut warnings = Vec::new();
    if !output.stderr.is_empty() {
        warnings.push(format!(
            "codegen command stderr: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }

    Ok(CodegenResult {
        code: stdout,
        warnings,
    })
}

#[derive(Debug, Serialize)]
struct HttpCodegenRequest {
    #[serde(rename = "imageBase64")]
    image_base64: String,
    #[serde(rename = "dataUrl")]
    data_url: String,
    stack: String,
    viewport: Option<Viewport>,
}

#[derive(Debug, Deserialize)]
struct HttpCodegenResponse {
    code: Option<String>,
    html: Option<String>,
    output: Option<String>,
    warnings: Option<Vec<String>>,
    summary: Option<String>,
    message: Option<String>,
}

async fn call_http_codegen(
    url: &str,
    screenshot_path: &Path,
    stack: &str,
    viewport: Option<Viewport>,
) -> Result<CodegenResult, DpcError> {
    let bytes = std::fs::read(screenshot_path)?;
    let encoded = BASE64_STANDARD.encode(bytes);
    let payload = HttpCodegenRequest {
        image_base64: encoded.clone(),
        data_url: format!("data:image/png;base64,{encoded}"),
        stack: stack.to_string(),
        viewport,
    };

    let client = reqwest::Client::new();
    let mut request = client.post(url).json(&payload);
    if let Ok(token) = std::env::var("DPC_CODEGEN_API_KEY") {
        if !token.trim().is_empty() {
            request = request.bearer_auth(token);
        }
    }

    let response = request.send().await.map_err(DpcError::Network)?;
    let status = response.status();
    let text = response.text().await.map_err(DpcError::Network)?;

    if !status.is_success() {
        return Err(DpcError::Config(format!(
            "Codegen HTTP {}: {}",
            status,
            text.trim()
        )));
    }

    let mut warnings = Vec::new();
    if let Ok(json) = serde_json::from_str::<HttpCodegenResponse>(&text) {
        if let Some(summary) = json.summary {
            warnings.push(summary);
        }
        if let Some(extra) = json.warnings {
            warnings.extend(extra);
        }
        if let Some(msg) = json.message {
            warnings.push(msg);
        }
        if let Some(code) = json.code.or(json.html).or(json.output) {
            if code.trim().is_empty() {
                return Err(DpcError::Config(
                    "Codegen response contained empty code".to_string(),
                ));
            }
            return Ok(CodegenResult { code, warnings });
        }
    }

    if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
        if let Some(code_val) = value
            .get("code")
            .or_else(|| value.get("html"))
            .or_else(|| value.get("output"))
            .or_else(|| value.get("result"))
        {
            if let Some(code) = code_val.as_str() {
                return Ok(CodegenResult {
                    code: code.to_string(),
                    warnings,
                });
            }
        }
    }

    if text.trim().is_empty() {
        return Err(DpcError::Config(
            "Codegen HTTP response was empty".to_string(),
        ));
    }

    warnings.push("Codegen response was not JSON; using raw body as code.".to_string());
    Ok(CodegenResult {
        code: text,
        warnings,
    })
}
