//! Figma integration for rendering designs to NormalizedView.
//!
//! This module provides:
//! - [`FigmaClient`] - HTTP client for the Figma REST API
//! - [`figma_to_normalized_view`] - Main conversion function
//! - [`FigmaRenderOptions`] - Configuration for Figma exports
//! - API types for parsing Figma JSON responses

pub mod api_types;
pub mod client;
pub mod conversion;
pub mod transform;

#[cfg(test)]
mod tests;

// Re-export primary public API
pub use client::{map_figma_error, FigmaClient, FigmaError};
pub use transform::FigmaRenderOptions;

// Re-export API types that may be needed externally
pub use api_types::{
    FigmaBoundingBox, FigmaColor, FigmaDocument, FigmaFile, FigmaImageExport, FigmaNodeData,
    FigmaNodesResponse, FigmaNodeWrapper, FigmaPaintData, FigmaTypeStyle, ImageFormat,
};

use crate::types::{NormalizedView, ResourceKind};
use crate::{DpcError, Result};
use image::{load_from_memory, GenericImageView};

/// Convert a Figma frame to a NormalizedView.
///
/// This function:
/// 1. Fetches the node data from Figma API
/// 2. Exports the frame as a PNG image
/// 3. Optionally resizes to the target viewport with letterboxing
/// 4. Normalizes bounding boxes to match the final image coordinates
pub async fn figma_to_normalized_view(
    client: &FigmaClient,
    options: &FigmaRenderOptions,
) -> Result<NormalizedView> {
    if options.scale <= 0.0 {
        return Err(DpcError::Config(
            "Figma export scale must be greater than zero".to_string(),
        ));
    }
    if options.file_key.trim().is_empty() {
        return Err(DpcError::Config(
            "Figma file key is required for export".to_string(),
        ));
    }
    if options.node_id.trim().is_empty() {
        return Err(DpcError::Config(
            "Figma node id is required for export".to_string(),
        ));
    }
    if options.output_path.as_os_str().is_empty() {
        return Err(DpcError::Config(
            "Figma export output_path is required".to_string(),
        ));
    }

    let nodes_response = client
        .get_file_nodes(&options.file_key, &[&options.node_id])
        .await
        .map_err(map_figma_error)?;

    let node = nodes_response
        .nodes
        .get(&options.node_id)
        .ok_or_else(|| DpcError::FigmaApi {
            status: None,
            message: format!("Node {} not found in Figma response", options.node_id),
        })?;

    let figma_snapshot =
        conversion::build_figma_snapshot(&options.file_key, &options.node_id, &node.document);

    let image_url = client
        .export_image(
            &options.file_key,
            &options.node_id,
            api_types::ImageFormat::Png,
            options.scale,
        )
        .await
        .map_err(map_figma_error)?;

    let bytes = client
        .download_image(&image_url)
        .await
        .map_err(map_figma_error)?;

    let decoded_image = load_from_memory(&bytes)?;
    let source_dimensions = decoded_image.dimensions();
    let (width, height, letterbox) = transform::finalize_figma_image(
        decoded_image,
        &options.output_path,
        options.viewport,
    )?;

    let root_bb = node
        .document
        .absolute_bounding_box
        .as_ref()
        .map(|bb| conversion::map_bounding_box(Some(bb)));
    let figma_snapshot =
        transform::normalize_figma_snapshot(figma_snapshot, root_bb, source_dimensions, &letterbox);

    Ok(NormalizedView {
        kind: ResourceKind::Figma,
        screenshot_path: options.output_path.clone(),
        width,
        height,
        dom: None,
        figma_tree: Some(figma_snapshot),
        ocr_blocks: None,
    })
}
