//! Letterbox transforms and image finalization for Figma exports.

use crate::image_loader::resize_with_letterbox;
use crate::types::{BoundingBox, FigmaNode, FigmaSnapshot};
use crate::{Result, Viewport};
use image::{DynamicImage, GenericImageView};
use std::fs;
use std::path::{Path, PathBuf};

/// Options for rendering a Figma frame to a normalized view.
#[derive(Debug, Clone)]
pub struct FigmaRenderOptions {
    pub file_key: String,
    pub node_id: String,
    pub output_path: PathBuf,
    pub viewport: Option<Viewport>,
    pub scale: f32,
}

impl Default for FigmaRenderOptions {
    fn default() -> Self {
        Self {
            file_key: String::new(),
            node_id: String::new(),
            output_path: PathBuf::new(),
            viewport: None,
            scale: 1.0,
        }
    }
}

/// Transform parameters for letterbox scaling.
#[derive(Debug, Clone, Copy)]
pub struct LetterboxTransform {
    pub scale: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

/// Compute letterbox transform parameters for fitting source into target dimensions.
pub fn compute_letterbox_transform(
    source_width: u32,
    source_height: u32,
    target_width: u32,
    target_height: u32,
) -> LetterboxTransform {
    let scale_w = target_width as f64 / source_width as f64;
    let scale_h = target_height as f64 / source_height as f64;
    let scale = scale_w.min(scale_h);
    let new_w = (source_width as f64 * scale).round() as u32;
    let new_h = (source_height as f64 * scale).round() as u32;
    let offset_x = ((target_width as i64 - new_w as i64) / 2) as f32;
    let offset_y = ((target_height as i64 - new_h as i64) / 2) as f32;

    LetterboxTransform {
        scale: scale as f32,
        offset_x,
        offset_y,
    }
}

/// Finalize a Figma image: optionally resize to viewport and save to disk.
///
/// Returns (width, height, letterbox_transform).
pub fn finalize_figma_image(
    img: DynamicImage,
    output_path: &Path,
    viewport: Option<Viewport>,
) -> Result<(u32, u32, LetterboxTransform)> {
    let (source_width, source_height) = img.dimensions();
    let (target_width, target_height) = viewport
        .map(|vp| (vp.width, vp.height))
        .unwrap_or((source_width, source_height));

    let letterbox =
        compute_letterbox_transform(source_width, source_height, target_width, target_height);

    let final_img = if viewport.is_some() {
        resize_with_letterbox(&img, target_width, target_height)
    } else {
        img
    };

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    final_img.save(output_path)?;

    Ok((target_width, target_height, letterbox))
}

/// Normalize Figma snapshot bounding boxes to match the final image coordinates.
///
/// Applies the letterbox transform to all node bounding boxes so they
/// align with the saved screenshot.
pub fn normalize_figma_snapshot(
    snapshot: FigmaSnapshot,
    root_bb: Option<BoundingBox>,
    source_dimensions: (u32, u32),
    letterbox: &LetterboxTransform,
) -> FigmaSnapshot {
    let (source_w, source_h) = source_dimensions;
    let (root_x, root_y, root_w, root_h) = root_bb
        .map(|bb| (bb.x, bb.y, bb.width, bb.height))
        .unwrap_or((0.0, 0.0, source_w as f32, source_h as f32));

    let scale_x = if root_w > 0.0 {
        source_w as f32 / root_w
    } else {
        1.0
    };
    let scale_y = if root_h > 0.0 {
        source_h as f32 / root_h
    } else {
        1.0
    };

    let mut nodes = Vec::with_capacity(snapshot.nodes.len());
    for node in snapshot.nodes {
        let rel_x = node.bounding_box.x - root_x;
        let rel_y = node.bounding_box.y - root_y;

        let scaled_x = rel_x * scale_x;
        let scaled_y = rel_y * scale_y;
        let scaled_w = node.bounding_box.width * scale_x;
        let scaled_h = node.bounding_box.height * scale_y;

        let final_x = scaled_x * letterbox.scale + letterbox.offset_x;
        let final_y = scaled_y * letterbox.scale + letterbox.offset_y;
        let final_w = scaled_w * letterbox.scale;
        let final_h = scaled_h * letterbox.scale;

        nodes.push(FigmaNode {
            bounding_box: BoundingBox {
                x: final_x,
                y: final_y,
                width: final_w,
                height: final_h,
            },
            ..node
        });
    }

    FigmaSnapshot { nodes, ..snapshot }
}
