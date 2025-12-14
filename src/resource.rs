use std::fs;
use std::path::Path;
use thiserror::Error;
use url::Url;

use crate::types::ResourceKind;

#[derive(Debug, Clone)]
pub struct ParsedResource {
    pub kind: ResourceKind,
    pub value: String,
    pub figma_info: Option<FigmaInfo>,
}

#[derive(Debug, Clone)]
pub struct FigmaInfo {
    pub file_key: String,
    pub node_id: Option<String>,
}

#[derive(Debug, Error)]
pub enum ResourceParseError {
    #[error("Invalid URL '{value}': {message}. Hint: include http(s):// and ensure the URL is well-formed.")]
    InvalidUrl { value: String, message: String },
    #[error("Figma URL missing file key in '{url}'. Hint: use https://www.figma.com/file/<FILE_KEY>/... and node-id if needed.")]
    FigmaMissingFileKey { url: String },
    #[error("Local file not found: {path}. Hint: check the path relative to the current working directory or use an absolute path.")]
    FileNotFound { path: String },
    #[error("Unsupported file extension '{extension}'. Supported image extensions: {supported}.")]
    UnsupportedExtension {
        extension: String,
        supported: String,
    },
}

const IMAGE_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "webp", "gif"];

pub fn parse_resource(
    value: &str,
    override_type: Option<ResourceKind>,
) -> Result<ParsedResource, ResourceParseError> {
    if let Some(kind) = override_type {
        return Ok(ParsedResource {
            kind,
            value: value.to_string(),
            figma_info: if kind == ResourceKind::Figma {
                parse_figma_url(value).ok()
            } else {
                None
            },
        });
    }

    if value.starts_with("http://") || value.starts_with("https://") {
        parse_url_resource(value)
    } else {
        parse_local_resource(value)
    }
}

fn parse_url_resource(value: &str) -> Result<ParsedResource, ResourceParseError> {
    let url = Url::parse(value).map_err(|e| ResourceParseError::InvalidUrl {
        value: value.to_string(),
        message: e.to_string(),
    })?;

    let host = url.host_str().unwrap_or("");
    if host.contains("figma.com") {
        let figma_info = parse_figma_url(value)?;
        Ok(ParsedResource {
            kind: ResourceKind::Figma,
            value: value.to_string(),
            figma_info: Some(figma_info),
        })
    } else {
        Ok(ParsedResource {
            kind: ResourceKind::Url,
            value: value.to_string(),
            figma_info: None,
        })
    }
}

fn parse_figma_url(value: &str) -> Result<FigmaInfo, ResourceParseError> {
    let url = Url::parse(value).map_err(|e| ResourceParseError::InvalidUrl {
        value: value.to_string(),
        message: e.to_string(),
    })?;

    let path_segments: Vec<&str> = url.path_segments().map(|c| c.collect()).unwrap_or_default();

    let file_key = path_segments
        .iter()
        .position(|&s| s == "file" || s == "design")
        .and_then(|i| path_segments.get(i + 1))
        .map(|s| s.to_string())
        .ok_or_else(|| ResourceParseError::FigmaMissingFileKey {
            url: value.to_string(),
        })?;

    let node_id = url
        .query_pairs()
        .find(|(k, _)| k == "node-id")
        .map(|(_, v)| v.replace('-', ":"));

    Ok(FigmaInfo { file_key, node_id })
}

fn parse_local_resource(value: &str) -> Result<ParsedResource, ResourceParseError> {
    let path = Path::new(value);

    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    if !IMAGE_EXTENSIONS.contains(&extension.as_str()) {
        if extension.is_empty() {
            return Err(ResourceParseError::UnsupportedExtension {
                extension: "no extension".to_string(),
                supported: IMAGE_EXTENSIONS.join(", "),
            });
        }

        return Err(ResourceParseError::UnsupportedExtension {
            extension,
            supported: IMAGE_EXTENSIONS.join(", "),
        });
    }

    if !path.exists() {
        return Err(ResourceParseError::FileNotFound {
            path: path.to_string_lossy().into_owned(),
        });
    }

    let metadata = fs::metadata(path).map_err(|_| ResourceParseError::FileNotFound {
        path: path.to_string_lossy().into_owned(),
    })?;
    if !metadata.is_file() {
        return Err(ResourceParseError::FileNotFound {
            path: path.to_string_lossy().into_owned(),
        });
    }

    Ok(ParsedResource {
        kind: ResourceKind::Image,
        value: value.to_string(),
        figma_info: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::Builder;

    fn temp_file_with_extension(ext: &str) -> tempfile::NamedTempFile {
        Builder::new()
            .suffix(&format!(".{}", ext))
            .tempfile()
            .expect("create temp file")
    }

    #[test]
    fn test_parse_http_url() {
        let res = parse_resource("http://localhost:3000/dashboard", None).unwrap();
        assert_eq!(res.kind, ResourceKind::Url);
        assert!(res.figma_info.is_none());
    }

    #[test]
    fn test_parse_https_url() {
        let res = parse_resource("https://example.com/page", None).unwrap();
        assert_eq!(res.kind, ResourceKind::Url);
    }

    #[test]
    fn test_parse_figma_url() {
        let url = "https://www.figma.com/file/ABC123/My-Design?node-id=12-34";
        let res = parse_resource(url, None).unwrap();
        assert_eq!(res.kind, ResourceKind::Figma);
        let info = res.figma_info.unwrap();
        assert_eq!(info.file_key, "ABC123");
        assert_eq!(info.node_id, Some("12:34".to_string()));
    }

    #[test]
    fn test_parse_figma_design_url() {
        let url = "https://www.figma.com/design/XYZ789/Another-Design?node-id=5-10";
        let res = parse_resource(url, None).unwrap();
        assert_eq!(res.kind, ResourceKind::Figma);
        let info = res.figma_info.unwrap();
        assert_eq!(info.file_key, "XYZ789");
        assert_eq!(info.node_id, Some("5:10".to_string()));
    }

    #[test]
    fn test_parse_figma_url_no_node_id() {
        let url = "https://www.figma.com/file/ABC123/My-Design";
        let res = parse_resource(url, None).unwrap();
        let info = res.figma_info.unwrap();
        assert_eq!(info.file_key, "ABC123");
        assert!(info.node_id.is_none());
    }

    #[test]
    fn test_parse_local_png() {
        let file = temp_file_with_extension("png");
        let res = parse_resource(file.path().to_str().unwrap(), None).unwrap();
        assert_eq!(res.kind, ResourceKind::Image);
    }

    #[test]
    fn test_parse_local_jpg() {
        let file = temp_file_with_extension("jpg");
        let res = parse_resource(file.path().to_str().unwrap(), None).unwrap();
        assert_eq!(res.kind, ResourceKind::Image);
    }

    #[test]
    fn test_parse_local_webp() {
        let file = temp_file_with_extension("webp");
        let res = parse_resource(file.path().to_str().unwrap(), None).unwrap();
        assert_eq!(res.kind, ResourceKind::Image);
    }

    #[test]
    fn test_parse_unsupported_extension() {
        let file = temp_file_with_extension("pdf");
        let res = parse_resource(file.path().to_str().unwrap(), None);
        assert!(matches!(
            res,
            Err(ResourceParseError::UnsupportedExtension { extension, .. })
                if extension == "pdf"
        ));
    }

    #[test]
    fn test_missing_local_image_errors() {
        let res = parse_resource("/tmp/does-not-exist.png", None);
        assert!(matches!(res, Err(ResourceParseError::FileNotFound { .. })));
    }

    #[test]
    fn test_override_type() {
        let res = parse_resource("/some/path", Some(ResourceKind::Url)).unwrap();
        assert_eq!(res.kind, ResourceKind::Url);
    }
}
