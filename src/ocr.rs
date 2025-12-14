//! OCR (Optical Character Recognition) module for extracting text from images.
//!
//! This module provides OCR text extraction using Tesseract via the `leptess` crate.
//! OCR is used as a fallback when DOM or Figma text data is unavailable (e.g., when
//! comparing two images directly).
//!
//! # Feature Flag
//!
//! This module requires the `ocr` feature flag to be enabled:
//!
//! ```toml
//! [dependencies]
//! dpc = { version = "0.1", features = ["ocr"] }
//! ```
//!
//! # System Requirements
//!
//! - Tesseract OCR must be installed on the system
//! - The `tessdata` directory must be accessible (typically at `/usr/share/tesseract-ocr/tessdata`
//!   or set via `TESSDATA_PREFIX` environment variable)

use std::path::Path;

use crate::types::OcrBlock;
#[cfg(feature = "ocr")]
use crate::types::BoundingBox;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OcrError {
    #[error("Failed to initialize OCR engine: {0}")]
    InitError(String),
    #[error("Failed to load image for OCR: {0}")]
    ImageLoadError(String),
    #[error("OCR processing failed: {0}")]
    ProcessingError(String),
    #[error("Tesseract not available - install Tesseract OCR or enable the 'ocr' feature")]
    NotAvailable,
}

/// Options for OCR text extraction.
#[derive(Debug, Clone)]
pub struct OcrOptions {
    /// Language code for Tesseract (default: "eng")
    pub language: String,
    /// Minimum confidence threshold for including text (0.0 - 1.0)
    pub min_confidence: f32,
}

impl Default for OcrOptions {
    fn default() -> Self {
        Self {
            language: "eng".to_string(),
            min_confidence: 0.5,
        }
    }
}

/// Extract text blocks from an image using OCR.
///
/// Returns a vector of `OcrBlock` containing the extracted text with
/// bounding boxes and confidence scores.
///
/// # Arguments
///
/// * `image_path` - Path to the image file
/// * `options` - OCR extraction options
///
/// # Errors
///
/// Returns `OcrError` if Tesseract fails to initialize or process the image.
#[cfg(feature = "ocr")]
pub fn extract_text_blocks(
    image_path: &Path,
    options: &OcrOptions,
) -> Result<Vec<OcrBlock>, OcrError> {
    use leptess::LepTess;

    let mut lt = LepTess::new(None, &options.language)
        .map_err(|e| OcrError::InitError(format!("{:?}", e)))?;

    lt.set_image(image_path)
        .map_err(|e| OcrError::ImageLoadError(format!("{:?}", e)))?;

    let mut blocks = Vec::new();

    let source_width = lt.get_source_dimensions().0 as f32;
    let source_height = lt.get_source_dimensions().1 as f32;

    let boxes = lt.get_component_boxes(leptess::capi::TessPageIteratorLevel_RIL_WORD, true);

    for b in boxes {
        lt.set_rectangle(
            b.x,
            b.y,
            b.w as i32,
            b.h as i32,
        );

        let text = lt.get_utf8_text().unwrap_or_default();
        let trimmed = text.trim();

        if trimmed.is_empty() {
            continue;
        }

        let confidence = lt.mean_text_conf() as f32 / 100.0;

        if confidence < options.min_confidence {
            continue;
        }

        blocks.push(OcrBlock {
            text: trimmed.to_string(),
            bounding_box: BoundingBox {
                x: b.x as f32,
                y: b.y as f32,
                width: b.w as f32,
                height: b.h as f32,
            },
            confidence: Some(confidence),
        });
    }

    // Reset to get line-level text if word-level produced nothing
    if blocks.is_empty() {
        lt.set_rectangle(0, 0, source_width as i32, source_height as i32);
        let full_text = lt.get_utf8_text().unwrap_or_default();

        for line in full_text.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                blocks.push(OcrBlock {
                    text: trimmed.to_string(),
                    bounding_box: BoundingBox {
                        x: 0.0,
                        y: 0.0,
                        width: source_width,
                        height: source_height,
                    },
                    confidence: Some(lt.mean_text_conf() as f32 / 100.0),
                });
            }
        }
    }

    Ok(blocks)
}

/// Stub implementation when OCR feature is disabled.
#[cfg(not(feature = "ocr"))]
pub fn extract_text_blocks(
    _image_path: &Path,
    _options: &OcrOptions,
) -> Result<Vec<OcrBlock>, OcrError> {
    Err(OcrError::NotAvailable)
}

/// Check if OCR is available in this build.
#[inline]
pub const fn is_available() -> bool {
    cfg!(feature = "ocr")
}

#[cfg(all(test, feature = "ocr"))]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_ocr_options_default() {
        let opts = OcrOptions::default();
        assert_eq!(opts.language, "eng");
        assert_eq!(opts.min_confidence, 0.5);
    }

    #[test]
    fn test_extract_nonexistent_file() {
        let result = extract_text_blocks(Path::new("/nonexistent/image.png"), &OcrOptions::default());
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod tests_no_feature {
    use super::*;

    #[test]
    fn test_is_available() {
        // Just verify it compiles and returns a bool
        let _ = is_available();
    }
}
