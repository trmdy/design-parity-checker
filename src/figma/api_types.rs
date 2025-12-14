//! Figma API response types for parsing JSON from the Figma REST API.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported image export formats.
#[derive(Debug, Clone, Copy)]
pub enum ImageFormat {
    Png,
    Jpg,
    Svg,
    Pdf,
}

impl ImageFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            ImageFormat::Png => "png",
            ImageFormat::Jpg => "jpg",
            ImageFormat::Svg => "svg",
            ImageFormat::Pdf => "pdf",
        }
    }
}

/// A Figma file response from the files endpoint.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FigmaFile {
    pub name: String,
    pub last_modified: String,
    pub version: String,
    pub document: FigmaDocument,
}

/// The root document of a Figma file.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FigmaDocument {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(default)]
    pub children: Vec<FigmaNodeData>,
}

/// Raw Figma node data from the API.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FigmaNodeData {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub node_type: String,
    #[serde(default)]
    pub children: Vec<FigmaNodeData>,
    pub absolute_bounding_box: Option<FigmaBoundingBox>,
    pub characters: Option<String>,
    pub style: Option<FigmaTypeStyle>,
    #[serde(default)]
    pub fills: Vec<FigmaPaintData>,
}

/// Bounding box coordinates from Figma.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FigmaBoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Typography style from Figma.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FigmaTypeStyle {
    pub font_family: Option<String>,
    pub font_size: Option<f32>,
    pub font_weight: Option<f32>,
    pub line_height_px: Option<f32>,
}

/// Paint/fill data from Figma.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FigmaPaintData {
    #[serde(rename = "type")]
    pub paint_type: String,
    pub color: Option<FigmaColor>,
    pub opacity: Option<f32>,
}

/// RGBA color from Figma (0.0-1.0 range).
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FigmaColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl FigmaColor {
    /// Convert to hex color string (e.g., "#ff7f00").
    pub fn to_hex(&self) -> String {
        format!(
            "#{:02x}{:02x}{:02x}",
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8
        )
    }
}

/// Response from the nodes endpoint.
#[derive(Debug, Deserialize)]
pub struct FigmaNodesResponse {
    pub nodes: HashMap<String, FigmaNodeWrapper>,
}

/// Wrapper containing the document for a node.
#[derive(Debug, Deserialize)]
pub struct FigmaNodeWrapper {
    pub document: FigmaNodeData,
}

/// Response from the images export endpoint.
#[derive(Debug, Deserialize)]
pub struct FigmaImageExport {
    pub images: HashMap<String, String>,
}
