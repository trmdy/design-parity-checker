//! Figma node tree building and mapping to internal types.

use crate::types::{
    BoundingBox, FigmaNode, FigmaPaint, FigmaPaintKind, FigmaSnapshot, TypographyStyle,
};

use super::api_types::{FigmaBoundingBox, FigmaNodeData, FigmaPaintData, FigmaTypeStyle};

/// Build a FigmaSnapshot from raw Figma API data.
pub fn build_figma_snapshot(file_key: &str, node_id: &str, root: &FigmaNodeData) -> FigmaSnapshot {
    let mut nodes = Vec::new();
    collect_figma_nodes(root, &mut nodes);

    FigmaSnapshot {
        file_key: file_key.to_string(),
        node_id: node_id.to_string(),
        name: Some(root.name.clone()),
        nodes,
    }
}

/// Recursively collect all Figma nodes into a flat list.
pub fn collect_figma_nodes(node: &FigmaNodeData, acc: &mut Vec<FigmaNode>) {
    let children_ids: Vec<String> = node.children.iter().map(|c| c.id.clone()).collect();
    for child in &node.children {
        collect_figma_nodes(child, acc);
    }

    acc.push(FigmaNode {
        id: node.id.clone(),
        name: Some(node.name.clone()),
        node_type: node.node_type.clone(),
        bounding_box: map_bounding_box(node.absolute_bounding_box.as_ref()),
        text: node.characters.clone(),
        typography: node.style.as_ref().map(map_typography),
        fills: node.fills.iter().filter_map(map_paint).collect(),
        children: children_ids,
    });
}

/// Map Figma typography style to internal TypographyStyle.
pub fn map_typography(style: &FigmaTypeStyle) -> TypographyStyle {
    TypographyStyle {
        font_family: style.font_family.clone(),
        font_size: style.font_size,
        font_weight: style.font_weight.map(|w| w.to_string()),
        line_height: style.line_height_px,
    }
}

/// Map Figma bounding box to internal BoundingBox.
pub fn map_bounding_box(bb: Option<&FigmaBoundingBox>) -> BoundingBox {
    bb.map(|b| BoundingBox {
        x: b.x,
        y: b.y,
        width: b.width,
        height: b.height,
    })
    .unwrap_or(BoundingBox {
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: 0.0,
    })
}

/// Map Figma paint data to internal FigmaPaint.
pub fn map_paint(paint: &FigmaPaintData) -> Option<FigmaPaint> {
    let kind = match paint.paint_type.to_lowercase().as_str() {
        "solid" => FigmaPaintKind::Solid,
        "image" => FigmaPaintKind::Image,
        v if v.starts_with("gradient") => FigmaPaintKind::Gradient,
        _ => FigmaPaintKind::Solid,
    };

    Some(FigmaPaint {
        kind,
        color: paint.color.as_ref().map(|c| c.to_hex()),
        opacity: paint.opacity,
    })
}
