use crate::common::document::document::Document;
use crate::common::document::node::AttrMap;
use crate::common::document::style::{Color, Display, FontWeight, StyleProperty, StylePropertyList, StyleValue, Unit};

pub mod node;
pub mod style;
pub mod document;


/// Creates an example HTML document with nodes, invisible nodes, attributes and style properties.
pub(crate) fn create_document() -> Document {
    let mut doc = Document::new();

    // --------------
    let mut style = StylePropertyList::new();
    style.set_property(StyleProperty::Width, StyleValue::Unit(200.0, Unit::Px));
    style.set_property(StyleProperty::Height, StyleValue::Unit(200.0, Unit::Px));
    style.set_property(StyleProperty::BorderLeftWidth, StyleValue::Unit(4.0, Unit::Px));
    style.set_property(StyleProperty::BorderRightWidth, StyleValue::Unit(4.0, Unit::Px));
    style.set_property(StyleProperty::BorderTopWidth, StyleValue::Unit(4.0, Unit::Px));
    style.set_property(StyleProperty::BorderBottomWidth, StyleValue::Unit(4.0, Unit::Px));

    let mut attrs = AttrMap::new();
    attrs.set("src", "image.jpg");
    attrs.set("alt", "image");

    let img_node_id = doc.new_element("img", Some(attrs), true, Some(style));

    // --------------
    let mut style = StylePropertyList::new();
    style.set_property(StyleProperty::Color, StyleValue::Color(Color::Named("red".to_string())));
    style.set_property(StyleProperty::Display, StyleValue::None);
    style.set_property(StyleProperty::FontWeight, StyleValue::FontWeight(FontWeight::Bolder));
    style.set_property(StyleProperty::FontSize, StyleValue::Unit(32.0, Unit::Px));
    style.set_property(StyleProperty::FontFamily, StyleValue::Keyword("Comic Sans MS".into()));

    let strong_node_id = doc.new_element("strong", None, false, Some(style.clone()));
    let strong_text_node_id = doc.new_text("This is some strong text", Some(style.clone()));
    doc.add_child(strong_node_id, strong_text_node_id);

    // --------------
    let mut style = StylePropertyList::new();
    style.set_property(StyleProperty::Display, StyleValue::Display(Display::Block));
    style.set_property(StyleProperty::MarginBlockStart, StyleValue::Unit(1.0, Unit::Em));
    style.set_property(StyleProperty::MarginBlockEnd, StyleValue::Unit(1.0, Unit::Em));
    style.set_property(StyleProperty::FontSize, StyleValue::Unit(32.0, Unit::Px));

    let mut attrs = AttrMap::new();
    attrs.set("class", "paragraph");

    let p_node_id = doc.new_element("p", Some(attrs), false, None);
    let p_text_node_id = doc.new_text("This is a paragraph with some text.. It may sound like this is not a lot of text, but it actually is. I don't know when exactly the text will wrap, but I assume this text is now long enough for that to happen.", Some(style));
    doc.add_child(p_node_id, strong_node_id);
    doc.add_child(p_node_id, p_text_node_id);
    doc.add_child(p_node_id, img_node_id);

    // --------------
    let mut style = StylePropertyList::new();
    style.set_property(StyleProperty::Display, StyleValue::Display(Display::Block));
    style.set_property(StyleProperty::FontSize, StyleValue::Unit(48.0, Unit::Px));
    style.set_property(StyleProperty::FontWeight, StyleValue::FontWeight(FontWeight::Bold));
    style.set_property(StyleProperty::FontFamily, StyleValue::Keyword("Verdana".into()));
    // style.set_property("margin-block-end", StyleValue::Unit(0.67, Unit::Em));

    style.set_property(StyleProperty::MarginBottom, StyleValue::Unit(10.0, Unit::Px));
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
    let h1_text_node_id = doc.new_text("header with some extra long text to see how it wraps", Some(style.clone()));
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
    // style.set_property(StyleProperty::Height, StyleValue::Unit(300.0, Unit::Px));
    // style.set_property(StyleProperty::Width, StyleValue::Unit(300.0, Unit::Px));
    // style.set_property(StyleProperty::MarginBlockEnd, StyleValue::Unit(0.67, Unit::Em));
    style.set_property(StyleProperty::MarginTop, StyleValue::Unit(25.0, Unit::Px));
    style.set_property(StyleProperty::MarginLeft, StyleValue::Unit(25.0, Unit::Px));
    style.set_property(StyleProperty::MarginBottom, StyleValue::Unit(25.0, Unit::Px));
    style.set_property(StyleProperty::MarginRight, StyleValue::Unit(25.0, Unit::Px));
    style.set_property(StyleProperty::PaddingTop, StyleValue::Unit(25.0, Unit::Px));
    style.set_property(StyleProperty::PaddingLeft, StyleValue::Unit(25.0, Unit::Px));
    style.set_property(StyleProperty::PaddingBottom, StyleValue::Unit(25.0, Unit::Px));
    style.set_property(StyleProperty::PaddingRight, StyleValue::Unit(25.0, Unit::Px));
    style.set_property(StyleProperty::BorderTopWidth, StyleValue::Unit(10.0, Unit::Px));
    style.set_property(StyleProperty::BorderLeftWidth, StyleValue::Unit(10.0, Unit::Px));
    style.set_property(StyleProperty::BorderBottomWidth, StyleValue::Unit(10.0, Unit::Px));
    style.set_property(StyleProperty::BorderRightWidth, StyleValue::Unit(10.0, Unit::Px));

    let mut attrs = AttrMap::new();
    attrs.set("lang", "en");
    let html_node_id = doc.new_element("html", Some(attrs), false, Some(style));
    doc.add_child(html_node_id, body_node_id);

    doc.set_root(html_node_id);
    doc
}
