use serde::Deserialize;
use std::collections::HashMap;
use crate::common::document::document::Document;
use crate::common::document::node::{AttrMap, NodeId};
use crate::common::document::style::{Color, Display, FontWeight, StyleProperty, StylePropertyList, StyleValue, Unit};

// This parses uses the tools/souper.py to load a JSON file and create a DOM from it. This allows us to render
// a webpage with minimal effort, and without connecting a whole html5 and css parser to it.

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DomNode {
    #[serde(default)]
    comment: Option<String>,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    tag: Option<String>,
    #[serde(default)]
    self_closing: bool,
    #[serde(default)]
    attributes: HashMap<String, String>,
    #[serde(default)]
    styles: HashMap<String, String>,
    #[serde(default)]
    children: Vec<DomNode>,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct DomRoot {
    tag: String,
    #[serde(default)]
    attributes: HashMap<String, String>,
    #[serde(default)]
    styles: HashMap<String, String>,
    children: Vec<DomNode>,
}

fn create_dom_from_json(doc: &mut Document, node: &DomNode, parent_id: Option<NodeId>) -> Option<NodeId> {
    let mut attrs = AttrMap::new();
    for (key, value) in &node.attributes {
        attrs.set(key, value);
    }

    let mut style = StylePropertyList::new();
    for (key, value) in &node.styles {
        match key.as_str() {
            "display" => style.set_property(StyleProperty::Display, parse_display(value)),
            "width" => style.set_property(StyleProperty::Width, parse_style_value(value)),
            "height" => style.set_property(StyleProperty::Height, parse_style_value(value)),
            "border-left-width" => style.set_property(StyleProperty::BorderLeftWidth, parse_style_value(value)),
            "border-right-width" => style.set_property(StyleProperty::BorderRightWidth, parse_style_value(value)),
            "border-top-width" => style.set_property(StyleProperty::BorderTopWidth, parse_style_value(value)),
            "border-bottom-width" => style.set_property(StyleProperty::BorderBottomWidth, parse_style_value(value)),
            "margin-top" => style.set_property(StyleProperty::MarginTop, parse_style_value(value)),
            "margin-left" => style.set_property(StyleProperty::MarginLeft, parse_style_value(value)),
            "color" => style.set_property(StyleProperty::Color, StyleValue::Color(Color::Named(value.to_string()))),
            "background-color" => style.set_property(StyleProperty::BackgroundColor, StyleValue::Color(Color::Named(value.to_string()))),
            "font-weight" => style.set_property(StyleProperty::FontWeight, parse_font_weight(value)),
            "font-size" => style.set_property(StyleProperty::FontSize, parse_style_value(value)),
            "font-family" => style.set_property(StyleProperty::FontFamily, StyleValue::Keyword(value.to_string())),
            _ => {}
        }
    }

    if let Some(text) = &node.text {
        return Some(doc.new_text(parent_id, text, Some(style)));
    }

    if let Some(comment) = &node.comment {
        return Some(doc.new_comment(parent_id, comment));
    }

    let Some(tag) = &node.tag else {
        eprintln!("Warning: Encountered node without a tag! {:?}", node);
        return None;
    };

    let node_id = doc.new_element(parent_id, &tag, Some(attrs), node.self_closing, Some(style.clone()));

    for child in &node.children {
        match create_dom_from_json(doc, child, Some(node_id)) {
            Some(child_node_id) => doc.add_child(node_id, child_node_id),
            None => {}
        }
    }

    Some(node_id)
}

fn parse_display(value: &String) -> StyleValue {
    match value.as_str() {
        "block" => StyleValue::Display(Display::Block),
        "inline" => StyleValue::Display(Display::Inline),
        "none" => StyleValue::Display(Display::None),
        _ => StyleValue::Keyword(value.to_string()),
    }
}

fn parse_style_value(value: &str) -> StyleValue {
    if let Ok(px_value) = value.replace("px", "").parse::<f32>() {
        StyleValue::Unit(px_value, Unit::Px)
    } else {
        StyleValue::Keyword(value.to_string())
    }
}

fn parse_font_weight(value: &str) -> StyleValue {
    match value {
        "bold" | "bolder" => StyleValue::FontWeight(FontWeight::Bolder),
        "lighter" => StyleValue::FontWeight(FontWeight::Lighter),
        "normal" => StyleValue::FontWeight(FontWeight::Normal),
        _ => {
            if let Ok(num) = value.parse::<f32>() {
                StyleValue::FontWeight(FontWeight::Number(num))
            } else {
                StyleValue::Keyword(value.to_string())
            }
        }
    }
}

pub fn document_from_json(path: &str) -> Document {
    let mut doc = Document::new();

    let json_data = std::fs::read_to_string(path).expect("Failed to read JSON file");
    let dom_root: DomRoot = serde_json::from_str(&json_data).expect("Failed to parse JSON");

    let root_node_id = doc.new_element(None, "DocumentRoot", None, false, None);
    for node in dom_root.children {
        if let Some(child_node_id) = create_dom_from_json(&mut doc, &node, Some(root_node_id)) {
            doc.add_child(root_node_id, child_node_id);
        }
    }

    doc.set_root(root_node_id);
    doc
}