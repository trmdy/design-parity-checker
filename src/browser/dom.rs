//! DOM snapshot types and conversion from raw Playwright output.

use crate::types::{BoundingBox, ComputedStyle, DomNode, DomSnapshot};
use std::collections::HashMap;

/// Raw script result with DOM snapshot from Playwright.
#[derive(Debug, serde::Deserialize)]
pub(crate) struct ScriptResultWithDom {
    pub status: String,
    pub dom: Option<RawDomSnapshot>,
}

/// Raw DOM snapshot as returned by the Playwright script.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawDomSnapshot {
    pub url: Option<String>,
    pub title: Option<String>,
    #[serde(default)]
    pub nodes: Vec<RawDomNode>,
}

/// Raw DOM node from Playwright output.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawDomNode {
    pub id: String,
    pub tag: String,
    #[serde(default)]
    pub children: Vec<String>,
    pub parent: Option<String>,
    #[serde(default)]
    pub attributes: HashMap<String, String>,
    pub text: Option<String>,
    pub bounding_box: RawBoundingBox,
    pub computed_style: Option<RawComputedStyle>,
}

/// Raw bounding box from Playwright output.
#[derive(Debug, serde::Deserialize)]
pub(crate) struct RawBoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Raw computed style from Playwright output.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawComputedStyle {
    pub font_family: Option<String>,
    pub font_size: Option<f32>,
    pub font_weight: Option<String>,
    pub line_height: Option<f32>,
    pub color: Option<String>,
    pub background_color: Option<String>,
    pub display: Option<String>,
    pub visibility: Option<String>,
    pub opacity: Option<f32>,
}

/// Converts raw DOM data from Playwright into the application's DomSnapshot type.
pub(crate) fn convert_raw_dom(dom_data: RawDomSnapshot) -> DomSnapshot {
    let nodes: Vec<DomNode> = dom_data
        .nodes
        .into_iter()
        .map(|raw| DomNode {
            id: raw.id,
            tag: raw.tag,
            children: raw.children,
            parent: raw.parent,
            attributes: raw.attributes,
            text: raw.text,
            bounding_box: BoundingBox {
                x: raw.bounding_box.x,
                y: raw.bounding_box.y,
                width: raw.bounding_box.width,
                height: raw.bounding_box.height,
            },
            computed_style: raw.computed_style.map(|s| ComputedStyle {
                font_family: s.font_family,
                font_size: s.font_size,
                font_weight: s.font_weight,
                line_height: s.line_height,
                color: s.color,
                background_color: s.background_color,
                display: s.display,
                visibility: s.visibility,
                opacity: s.opacity,
            }),
        })
        .collect();

    DomSnapshot {
        url: dom_data.url,
        title: dom_data.title,
        nodes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_dom_snapshot_deserializes_correctly() {
        let json = r#"{
            "url": "https://example.com",
            "title": "Example Page",
            "nodes": [{
                "id": "node-0",
                "tag": "div",
                "children": ["node-1"],
                "parent": null,
                "attributes": {"class": "container"},
                "text": "Hello",
                "boundingBox": {"x": 0, "y": 0, "width": 100, "height": 50},
                "computedStyle": {
                    "fontFamily": "Arial",
                    "fontSize": 16.0,
                    "fontWeight": "400",
                    "lineHeight": 24.0,
                    "color": "rgb(0, 0, 0)",
                    "backgroundColor": "rgb(255, 255, 255)",
                    "display": "block",
                    "visibility": "visible",
                    "opacity": 0.5
                }
            }]
        }"#;

        let snapshot: RawDomSnapshot = serde_json::from_str(json).unwrap();
        assert_eq!(snapshot.url, Some("https://example.com".to_string()));
        assert_eq!(snapshot.title, Some("Example Page".to_string()));
        assert_eq!(snapshot.nodes.len(), 1);

        let node = &snapshot.nodes[0];
        assert_eq!(node.id, "node-0");
        assert_eq!(node.tag, "div");
        assert_eq!(node.children, vec!["node-1"]);
        assert!(node.parent.is_none());
        assert_eq!(node.attributes.get("class"), Some(&"container".to_string()));
        assert_eq!(node.text, Some("Hello".to_string()));
        assert_eq!(node.bounding_box.width, 100.0);

        let style = node.computed_style.as_ref().unwrap();
        assert_eq!(style.font_family, Some("Arial".to_string()));
        assert_eq!(style.font_size, Some(16.0));
        assert_eq!(style.display.as_deref(), Some("block"));
        assert_eq!(style.visibility.as_deref(), Some("visible"));
        assert_eq!(style.opacity, Some(0.5));
    }

    #[test]
    fn script_result_with_dom_deserializes() {
        let json = r#"{
            "status": "ok",
            "dom": {
                "url": "https://test.com",
                "title": "Test",
                "nodes": []
            }
        }"#;

        let result: ScriptResultWithDom = serde_json::from_str(json).unwrap();
        assert_eq!(result.status, "ok");
        assert!(result.dom.is_some());
        let dom = result.dom.unwrap();
        assert_eq!(dom.url, Some("https://test.com".to_string()));
        assert!(dom.nodes.is_empty());
    }

    #[test]
    fn convert_raw_dom_copies_style_fields() {
        let raw = RawDomSnapshot {
            url: Some("https://example.com".into()),
            title: Some("Example".into()),
            nodes: vec![RawDomNode {
                id: "n1".into(),
                tag: "div".into(),
                children: vec![],
                parent: None,
                attributes: HashMap::new(),
                text: Some("hello".into()),
                bounding_box: RawBoundingBox {
                    x: 1.0,
                    y: 2.0,
                    width: 3.0,
                    height: 4.0,
                },
                computed_style: Some(RawComputedStyle {
                    font_family: Some("Arial".into()),
                    font_size: Some(12.0),
                    font_weight: Some("700".into()),
                    line_height: Some(16.0),
                    color: Some("rgb(0,0,0)".into()),
                    background_color: Some("rgb(255,255,255)".into()),
                    display: Some("block".into()),
                    visibility: Some("visible".into()),
                    opacity: Some(0.8),
                }),
            }],
        };

        let snapshot = convert_raw_dom(raw);
        let node = snapshot.nodes.first().unwrap();

        assert_eq!(node.text.as_deref(), Some("hello"));
        let style = node.computed_style.as_ref().unwrap();
        assert_eq!(style.font_family.as_deref(), Some("Arial"));
        assert_eq!(style.display.as_deref(), Some("block"));
        assert_eq!(style.visibility.as_deref(), Some("visible"));
        assert_eq!(style.opacity, Some(0.8));
    }
}
