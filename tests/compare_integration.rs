use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use dpc_lib::{DpcOutput, ResourceKind};

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

fn run_compare(args: &[&str], envs: &[(&str, &str)]) -> Output {
    let mut cmd = Command::new(bin_path());
    cmd.args(args);

    for (k, v) in envs {
        cmd.env(k, v);
    }

    cmd.output().expect("run compare command")
}

fn parse_output(stdout: &[u8]) -> DpcOutput {
    serde_json::from_slice(stdout).expect("compare output should be valid JSON")
}

#[test]
fn image_inputs_emit_json_and_pass() {
    let output = run_compare(
        &[
            "compare",
            "--ref",
            asset("ref.png").to_str().unwrap(),
            "--impl",
            asset("impl_identical.png").to_str().unwrap(),
            "--format",
            "json",
            "--threshold",
            "0.90",
        ],
        &[],
    );

    assert!(
        output.status.success(),
        "expected success exit, got {:?}",
        output.status.code()
    );

    match parse_output(&output.stdout) {
        DpcOutput::Compare(out) => {
            assert!(out.passed, "expected passed=true");
            assert!(
                out.similarity >= out.threshold,
                "similarity {} should meet/exceed threshold {}",
                out.similarity,
                out.threshold
            );
            assert!(
                out.summary.as_ref().map_or(false, |s| !s.top_issues.is_empty()),
                "summary should include top issues"
            );
        }
        other => panic!("expected compare output, got {:?}", other),
    }
}

#[test]
fn image_inputs_fail_when_below_threshold() {
    let output = run_compare(
        &[
            "compare",
            "--ref",
            asset("ref.png").to_str().unwrap(),
            "--impl",
            asset("impl_different.png").to_str().unwrap(),
            "--format",
            "json",
            "--threshold",
            "0.99",
        ],
        &[],
    );

    assert_eq!(output.status.code(), Some(1), "should exit with failure code");

    match parse_output(&output.stdout) {
        DpcOutput::Compare(out) => {
            assert!(
                !out.passed,
                "expected passed=false when similarity below threshold"
            );
            assert!(
                out.similarity < out.threshold,
                "similarity {} should be below threshold {}",
                out.similarity,
                out.threshold
            );
        }
        other => panic!("expected compare output, got {:?}", other),
    }
}

#[test]
fn url_inputs_use_mock_renderer() {
    let output = run_compare(
        &[
            "compare",
            "--ref",
            "https://example.com/design",
            "--impl",
            "https://example.com/build",
            "--format",
            "json",
            "--threshold",
            "0.99",
        ],
        &[
            ("DPC_MOCK_RENDER_REF", asset("ref.png").to_str().unwrap()),
            (
                "DPC_MOCK_RENDER_IMPL",
                asset("impl_different.png").to_str().unwrap(),
            ),
        ],
    );

    assert_eq!(output.status.code(), Some(1), "URL mock run should fail threshold");

    match parse_output(&output.stdout) {
        DpcOutput::Compare(out) => {
            assert!(matches!(out.ref_resource.kind, ResourceKind::Url));
            assert!(matches!(out.impl_resource.kind, ResourceKind::Url));
            assert!(
                !out.passed,
                "expected failure when mock images differ significantly"
            );
        }
        other => panic!("expected compare output, got {:?}", other),
    }
}

#[test]
fn figma_inputs_use_mock_renderer() {
    let output = run_compare(
        &[
            "compare",
            "--ref",
            "https://www.figma.com/file/FILE123/Mock?node-id=1-2",
            "--impl",
            "https://www.figma.com/file/FILE123/Mock?node-id=2-3",
            "--format",
            "json",
            "--threshold",
            "0.90",
        ],
        &[
            ("DPC_MOCK_RENDER_REF", asset("ref.png").to_str().unwrap()),
            (
                "DPC_MOCK_RENDER_IMPL",
                asset("impl_identical.png").to_str().unwrap(),
            ),
            ("FIGMA_TOKEN", "dummy-token"),
        ],
    );

    assert!(
        output.status.success(),
        "figma mock run should pass: {:?}",
        output.status.code()
    );

    match parse_output(&output.stdout) {
        DpcOutput::Compare(out) => {
            assert!(matches!(out.ref_resource.kind, ResourceKind::Figma));
            assert!(matches!(out.impl_resource.kind, ResourceKind::Figma));
            assert!(out.passed, "expected passing result for matching mock images");
        }
        other => panic!("expected compare output, got {:?}", other),
    }
}
