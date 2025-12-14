use dpc_lib::DpcError;

#[test]
fn config_error_display_includes_message() {
    let err = DpcError::Config("missing viewport".to_string());

    assert_eq!(format!("{}", err), "Configuration error: missing viewport");
}

#[test]
fn io_error_display_wraps_source() {
    let io_err = std::io::Error::other("disk full");
    let err: DpcError = io_err.into();
    let rendered = format!("{}", err);

    assert!(rendered.starts_with("IO error: "));
    assert!(rendered.contains("disk full"));
}

#[test]
fn figma_api_helper_includes_status_and_message() {
    let err = DpcError::figma_api(Some(reqwest::StatusCode::NOT_FOUND), "not found");

    assert_eq!(
        format!("{}", err),
        "Figma API error (status: Some(404)): not found"
    );
}

#[test]
fn figma_api_helper_handles_missing_status() {
    let err = DpcError::figma_api(None, "missing token");

    assert_eq!(
        format!("{}", err),
        "Figma API error (status: None): missing token"
    );
}

#[test]
fn metric_helper_uses_message() {
    let err = DpcError::metric("bad metric output");

    assert_eq!(
        format!("{}", err),
        "Metric computation error: bad metric output"
    );
}
