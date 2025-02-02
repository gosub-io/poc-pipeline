use crate::document::document::Document;
use crate::document::node::{Node, NodeType};

pub struct RenderNode {
    pub node_id: usize,
    pub children: Vec<RenderNode>,
}

/// A rendertree holds both the DOM and the render tree. This tree holds all the visible nodes in
/// the DOM.
pub(crate) struct RenderTree {
    pub doc: Document,
    pub root: RenderNode,
}

impl RenderTree {
    pub(crate) fn count_elements(&self) -> usize {
        fn count_elements_node(node: &RenderNode) -> usize {
            let mut count = 1;
            for child in &node.children {
                count += count_elements_node(child);
            }
            count
        }

        count_elements_node(&self.root)
    }
}

impl RenderTree {
    pub(crate) fn print(&self) {
        self.print_node(&self.root, 0);
    }

    fn print_node(&self, node: &RenderNode, level: usize) {
        let indent = " ".repeat(level * 4);
        println!("{}{}", indent, node.node_id);
        for child in &node.children {
            self.print_node(child, level + 1);
        }
    }
}

const INVISIBLE_ELEMENTS: [&str; 6] = [ "head",  "style",  "script",  "meta",  "link",  "title" ];

impl RenderTree {
    pub(crate) fn new(doc: Document) -> Self {
        RenderTree {
            doc,
            root: RenderNode { node_id: 0, children: vec![] },
        }
    }

    pub fn parse(&mut self) {
        if self.doc.root.is_none() {
            panic!("Document has no root node");
        }

        let doc = &self.doc;
        match self.build_rendertree(&doc.root.as_ref().unwrap()) {
            Some(render_node) => self.root = render_node,
            None => panic!("Failed to build rendertree"),
        }
    }

    fn is_visible(&self, node: &Node) -> bool {
        match &node.node_type {
            NodeType::Text(_) => true,
            NodeType::Element(element) => {
                // Check element name
                if INVISIBLE_ELEMENTS.contains(&element.tag_name.as_str()) {
                    return false;
                }

                // Check attributes
                if let Some(attr) = element.get_attribute("hidden") {
                    if attr == "true" {
                        return false;
                    }
                }

                if element.get_style("display") == Some(&"none".to_string()) {
                    return false;
                }

                true
            }
        }
    }

    fn build_rendertree(&self, node: &Node) -> Option<RenderNode> {
        if !self.is_visible(node) {
            return None;
        }

        let mut render_node = RenderNode {
            node_id: node.node_id,
            children: Vec::new(),
        };

        for child in &node.children {
            if let Some(render_child) = self.build_rendertree(child) {
                render_node.children.push(render_child);
            }
        }

        Some(render_node)
    }
}