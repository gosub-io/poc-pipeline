use render_tree::RenderTree;
use crate::document::document::Document;
use crate::document::node::{AttrMap, Node};
use crate::document::style::StylePropertyList;

#[allow(unused)]
mod document;
#[allow(unused)]
mod render_tree;

fn main() {
    let doc = create_document();
    let mut output = String::new();
    doc.print_tree(&mut output).unwrap();
    println!("{}", output);


    let mut render_tree = RenderTree::new(doc);
    render_tree.parse();
    render_tree.print();

    let doc_element_count = render_tree.doc.count_elements();
    let render_tree_element_count = render_tree.count_elements();

    println!("{:.2}% of the dom elements removed", (1.0 - (render_tree_element_count as f64 / doc_element_count as f64)) * 100.0);
}


fn create_document() -> Document {
    let mut doc = Document::new();

    let mut attrs = AttrMap::new();
    attrs.set("src", "image.jpg");
    attrs.set("alt", "image");
    let img_node = Node::new_element(&doc, "img".to_string(), Some(attrs), true, None);

    let mut style = StylePropertyList::new();
    style.set_property("color", "red");
    style.set_property("display", "none");
    let mut strong_node = Node::new_element(&doc, "strong".to_string(), None, false, Some(style));
    strong_node.children.push(Node::new_text(&doc, "strong".to_string()));

    let mut attrs = AttrMap::new();
    attrs.set("class", "paragraph");
    let mut p_node = Node::new_element(&doc, "p".to_string(), Some(attrs), false, None);
    p_node.children.push(Node::new_text(&doc, "paragraph".to_string()));
    p_node.children.push(strong_node);
    p_node.children.push(img_node);

    let mut attrs = AttrMap::new();
    attrs.set("class", "title");
    attrs.set("data-alpine", "x-wrap");
    let mut h1_node = Node::new_element(&doc, "h1".to_string(), Some(attrs), false, None);
    h1_node.children.push(Node::new_text(&doc, "header".to_string()));

    let mut attrs = AttrMap::new();
    attrs.set("src", "script.js");
    attrs.set("type", "text/javascript");
    attrs.set("async", "true");
    let script_node = Node::new_element(&doc, "script".to_string(), Some(attrs), true, None);

    let mut body_node = Node::new_element(&doc, "body".to_string(), None, false, None);
    body_node.children.push(h1_node);
    body_node.children.push(script_node);
    body_node.children.push(p_node);

    let mut attrs = AttrMap::new();
    attrs.set("lang", "en");
    let mut html_node = Node::new_element(&doc, "html".to_string(), Some(attrs), false, None);
    html_node.children.push(body_node);

    doc.set_root(html_node);

    doc
}
