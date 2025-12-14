use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use dpc_lib::DpcOutput;

fn bin_path() -> PathBuf {
    std::env::var("CARGO_BIN_EXE_dpc")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("target")
                .join("debug")
                .join(if cfg!(windows) { "dpc.exe" } else { "dpc" })
        })
}

fn asset(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("test_assets")
        .join(name)
}

fn run_cmd(args: &[&str]) -> Output {
    Command::new(bin_path())
        .args(args)
        .output()
        .expect("run dpc command")
}

fn parse_json(stdout: &[u8]) -> DpcOutput {
    serde_json::from_slice(stdout).expect("output should be valid JSON")
}

#[test]
fn generate_code_stub_returns_not_implemented() {
    let output = run_cmd(&[
        "generate-code",
        "--input",
        asset("ref.png").to_str().unwrap(),
        "--stack",
        "html+tailwind",
        "--format",
        "json",
    ]);

    assert!(
        output.status.success(),
        "generate-code stub should exit 0, got {:?}",
        output.status.code()
    );

    match parse_json(&output.stdout) {
        DpcOutput::GenerateCode(out) => {
            assert_eq!(out.input.kind, dpc_lib::ResourceKind::Image);
            assert!(
                out.summary
                    .as_ref()
                    .and_then(|s| s.top_issues.first())
                    .map_or(false, |t| t
                        .to_ascii_lowercase()
                        .contains("not implemented")),
                "summary.topIssues should include not-implemented note"
            );
            assert!(
                out.code.is_none(),
                "code should be absent/null for stub output"
            );
        }
        other => panic!("expected generate-code output, got {:?}", other),
    }
}

#[test]
fn quality_stub_returns_not_implemented() {
    let output = run_cmd(&[
        "quality",
        "--input",
        asset("ref.png").to_str().unwrap(),
        "--format",
        "json",
    ]);

    assert!(
        output.status.success(),
        "quality stub should exit 0, got {:?}",
        output.status.code()
    );

    match parse_json(&output.stdout) {
        DpcOutput::Quality(out) => {
            assert_eq!(out.input.kind, dpc_lib::ResourceKind::Image);
            assert!(
                !out.findings.is_empty(),
                "quality stub should emit at least one finding"
            );
            let first = &out.findings[0];
            assert_eq!(first.finding_type, "not_implemented");
            assert!(matches!(first.severity, dpc_lib::FindingSeverity::Info));
            assert!(
                first
                    .message
                    .to_ascii_lowercase()
                    .contains("not implemented"),
                "expected not-implemented message in finding"
            );
        }
        other => panic!("expected quality output, got {:?}", other),
    }
}
