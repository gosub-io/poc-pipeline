use crate::document::document::Document;
use crate::document::node::{AttrMap, Node};
use crate::document::style::{Color, Display, FontWeight, StylePropertyList, StyleValue, Unit};

pub mod node;
pub mod style;
pub mod document;


/// Creates an example HTML document with nodes, invisible nodes, attributes and style properties.
pub(crate) fn create_document() -> Document {
    let mut doc = Document::new();

    // --------------
    let mut style = StylePropertyList::new();
    style.set_property("width", StyleValue::Unit(450.0, Unit::Px));
    style.set_property("height", StyleValue::Unit(300.0, Unit::Px));
    style.set_property("border-left-width", StyleValue::Unit(4.0, Unit::Px));
    style.set_property("border-right-width", StyleValue::Unit(4.0, Unit::Px));
    style.set_property("border-top-width", StyleValue::Unit(4.0, Unit::Px));
    style.set_property("border-bottom-width", StyleValue::Unit(4.0, Unit::Px));

    let mut attrs = AttrMap::new();
    attrs.set("src", "image.jpg");
    attrs.set("alt", "image");

    let img_node_id = doc.new_element("img", Some(attrs), true, Some(style));

    // --------------
    let mut style = StylePropertyList::new();
    style.set_property("color", StyleValue::Color(Color::Named("red".to_string())));
    style.set_property("display", StyleValue::None);
    style.set_property("font-weight", StyleValue::FontWeight(FontWeight::Bolder));

    let strong_node_id = doc.new_element("strong", None, false, Some(style));
    let strong_text_node_id = doc.new_text("strong");
    doc.add_child(strong_node_id, strong_text_node_id);

    // --------------
    let mut style = StylePropertyList::new();
    style.set_property("display", StyleValue::Display(Display::Block));
    style.set_property("margin-block-start", StyleValue::Unit(1.0, Unit::Em));
    style.set_property("margin-block-end", StyleValue::Unit(1.0, Unit::Em));

    let mut attrs = AttrMap::new();
    attrs.set("class", "paragraph");

    let p_node_id = doc.new_element("p", Some(attrs), false, None);
    let p_text_node_id = doc.new_text("paragraph");
    doc.add_child(p_node_id, strong_node_id);
    doc.add_child(p_node_id, img_node_id);

    // --------------
    let mut style = StylePropertyList::new();
    style.set_property("display", StyleValue::Display(Display::Block));
    style.set_property("font-size", StyleValue::Unit(48.0, Unit::Em));
    style.set_property("font-weight", StyleValue::FontWeight(FontWeight::Bold));
    // style.set_property("margin-block-end", StyleValue::Unit(0.67, Unit::Em));

    style.set_property("margin-bottom", StyleValue::Unit(10.0, Unit::Px));
    // style.set_property("margin-top", StyleValue::Unit(10.0, Unit::Px));
    // style.set_property("margin-left", StyleValue::Unit(10.0, Unit::Px));
    // style.set_property("margin-right", StyleValue::Unit(10.0, Unit::Px));

    // style.set_property("padding-top", StyleValue::Unit(10.0, Unit::Px));
    // style.set_property("padding-left", StyleValue::Unit(10.0, Unit::Px));
    // style.set_property("padding-bottom", StyleValue::Unit(10.0, Unit::Px));
    // style.set_property("padding-right", StyleValue::Unit(10.0, Unit::Px));
    // style.set_property("border-top-width", StyleValue::Unit(4.0, Unit::Px));
    // style.set_property("border-left-width", StyleValue::Unit(4.0, Unit::Px));
    // style.set_property("border-bottom-width", StyleValue::Unit(4.0, Unit::Px));
    // style.set_property("border-right-width", StyleValue::Unit(4.0, Unit::Px));

    let mut attrs = AttrMap::new();
    attrs.set("class", "title");
    attrs.set("data-alpine", "x-wrap");

    let h1_node_id = doc.new_element("h1", Some(attrs.clone()), false, Some(style.clone()));
    let h1_text_node_id = doc.new_text("header");
    doc.add_child(h1_node_id, h1_text_node_id);

    // --------------
    let mut attrs = AttrMap::new();
    attrs.set("src", "script.js");
    attrs.set("type", "text/javascript");
    attrs.set("async", "true");
    let script_node_id = doc.new_element("script", Some(attrs), true, None);

    // --------------
    let body_node_id = doc.new_element("body", None, false, None);
    doc.add_child(body_node_id, h1_node_id);
    doc.add_child(body_node_id, script_node_id);
    doc.add_child(body_node_id, p_node_id);

    // --------------
    let mut style = StylePropertyList::new();
    // style.set_property("height", StyleValue::Unit(300.0, Unit::Px));
    // style.set_property("width", StyleValue::Unit(300.0, Unit::Px));
    // style.set_property("margin-block-end", StyleValue::Unit(0.67, Unit::Em));
    style.set_property("margin-top", StyleValue::Unit(25.0, Unit::Px));
    style.set_property("margin-left", StyleValue::Unit(25.0, Unit::Px));
    style.set_property("margin-bottom", StyleValue::Unit(25.0, Unit::Px));
    style.set_property("margin-right", StyleValue::Unit(25.0, Unit::Px));
    style.set_property("padding-top", StyleValue::Unit(25.0, Unit::Px));
    style.set_property("padding-left", StyleValue::Unit(25.0, Unit::Px));
    style.set_property("padding-bottom", StyleValue::Unit(25.0, Unit::Px));
    style.set_property("padding-right", StyleValue::Unit(25.0, Unit::Px));
    style.set_property("border-top-width", StyleValue::Unit(10.0, Unit::Px));
    style.set_property("border-left-width", StyleValue::Unit(10.0, Unit::Px));
    style.set_property("border-bottom-width", StyleValue::Unit(10.0, Unit::Px));
    style.set_property("border-right-width", StyleValue::Unit(10.0, Unit::Px));

    let mut attrs = AttrMap::new();
    attrs.set("lang", "en");
    let html_node_id = doc.new_element("html", Some(attrs), false, Some(style));
    doc.add_child(html_node_id, body_node_id);

    doc.set_root(html_node_id);
    doc
}
