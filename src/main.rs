use render_tree::RenderTree;
use crate::document::document::Document;
use crate::document::node::{AttrMap, Node};
use crate::document::style::{Color, Display, StylePropertyList, StyleValue, Unit, FontWeight};

#[allow(unused)]
mod document;
#[allow(unused)]
mod render_tree;
#[allow(unused)]
mod layouter;

fn main() {
    // --------------------------------------------------------------------
    let doc = create_document();
    let mut output = String::new();
    doc.print_tree(&mut output).unwrap();
    println!("{}", output);

    // --------------------------------------------------------------------
    let mut render_tree = RenderTree::new(doc);
    render_tree.parse();
    render_tree.print();

    let doc_element_count = render_tree.doc.count_elements();
    let render_tree_element_count = render_tree.count_elements();

    println!("{:.2}% of the dom elements removed", (1.0 - (render_tree_element_count as f64 / doc_element_count as f64)) * 100.0);

    // --------------------------------------------------------------------
    let mut layouter = layouter::Layouter::new(render_tree);
    layouter.generate(layouter::Size { width: 800.0, height: 600.0 });
}


fn create_document() -> Document {
    let mut doc = Document::new();

    // --------------
    let mut style = StylePropertyList::new();
    style.set_property("width", StyleValue::Unit(150.0, Unit::Px));
    style.set_property("height", StyleValue::Unit(100.0, Unit::Px));

    let mut attrs = AttrMap::new();
    attrs.set("src", "image.jpg");
    attrs.set("alt", "image");

    let img_node = Node::new_element(&doc, "img".to_string(), Some(attrs), true, Some(style));

    // --------------
    let mut style = StylePropertyList::new();
    style.set_property("color", StyleValue::Color(Color::Named("red".to_string())));
    style.set_property("display", StyleValue::None);
    style.set_property("font-weight", StyleValue::FontWeight(FontWeight::Bolder));

    let mut strong_node = Node::new_element(&doc, "strong".to_string(), None, false, Some(style));
    strong_node.children.push(Node::new_text(&doc, "strong".to_string()));

    // --------------
    let mut style = StylePropertyList::new();
    style.set_property("display", StyleValue::Display(Display::Block));
    style.set_property("margin-block-start", StyleValue::Unit(1.0, Unit::Em));
    style.set_property("margin-block-end", StyleValue::Unit(1.0, Unit::Em));

    let mut attrs = AttrMap::new();
    attrs.set("class", "paragraph");

    let mut p_node = Node::new_element(&doc, "p".to_string(), Some(attrs), false, None);
    p_node.children.push(Node::new_text(&doc, "paragraph".to_string()));
    p_node.children.push(strong_node);
    p_node.children.push(img_node);


    // --------------
    let mut style = StylePropertyList::new();
    style.set_property("display", StyleValue::Display(Display::Block));
    style.set_property("font-size", StyleValue::Unit(2.0, Unit::Em));
    style.set_property("font-weight", StyleValue::FontWeight(FontWeight::Bold));
    style.set_property("margin-block", StyleValue::Unit(0.67, Unit::Em));

    let mut attrs = AttrMap::new();
    attrs.set("class", "title");
    attrs.set("data-alpine", "x-wrap");

    let mut h1_node = Node::new_element(&doc, "h1".to_string(), Some(attrs), false, None);
    h1_node.children.push(Node::new_text(&doc, "header".to_string()));

    // --------------
    let mut attrs = AttrMap::new();
    attrs.set("src", "script.js");
    attrs.set("type", "text/javascript");
    attrs.set("async", "true");
    let script_node = Node::new_element(&doc, "script".to_string(), Some(attrs), true, None);

    // --------------
    let mut body_node = Node::new_element(&doc, "body".to_string(), None, false, None);
    body_node.children.push(h1_node);
    body_node.children.push(script_node);
    body_node.children.push(p_node);

    // --------------
    let mut attrs = AttrMap::new();
    attrs.set("lang", "en");
    let mut html_node = Node::new_element(&doc, "html".to_string(), Some(attrs), false, None);
    html_node.children.push(body_node);


    doc.set_root(html_node);
    doc
}
