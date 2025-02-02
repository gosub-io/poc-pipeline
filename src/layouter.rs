use crate::render_tree::{RenderNode, RenderTree};
use taffy::prelude::*;
use crate::document::node::NodeType;
use crate::document::style::{StyleValue, Unit};

pub(crate) struct Size {
    pub(crate) width: f64,
    pub(crate) height: f64,
}

pub(crate) struct Layouter {
    render_tree: RenderTree,
}

impl Layouter {
    pub fn new(render_tree: RenderTree) -> Layouter {
        Layouter {
            render_tree,
        }
    }

    pub fn generate(&mut self, viewport: Size) {
        let taffy_tree = generate_taffy_tree(&self.render_tree, viewport);
    }
}

fn generate_taffy_tree(render_tree: &RenderTree, viewport: Size) -> TaffyTree {
    let mut tree: TaffyTree<()> = TaffyTree::new();

    let root_id = generate_node(&mut tree, &render_tree, &render_tree.root);
    if root_id.is_none() {
        panic!("Failed to generate root node render tree");
    }

    // let size = taffy::geometry::Size::new(viewport.width as f32, viewport.height as f32);
    tree.compute_layout(root_id.unwrap(), taffy::geometry::Size::MAX_CONTENT).unwrap();
    tree.print_tree(root_id.unwrap());

    tree
}

fn generate_node(tree: &mut TaffyTree<()>, render_tree: &RenderTree, render_node: &RenderNode) -> Option<NodeId> {
    let mut style = Style {
        display: Display::Block,
        ..Default::default()
    };

    let Some(dom_node) = render_tree.doc.get_node_by_id(render_node.node_id) else {
        return None;
    };

    match &dom_node.node_type {
        NodeType::Element(data) => {
            if let Some(width) = data.get_style("width") {
                match width {
                    StyleValue::Unit(value, unit) => {
                        match unit {
                            Unit::Px => style.size.width = Dimension::Length(*value),
                            Unit::Percent => style.size.width = Dimension::Percent(*value),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            if let Some(height) = data.get_style("height") {
                match height {
                    StyleValue::Unit(value, unit) => {
                        match unit {
                            Unit::Px => style.size.height = Dimension::Length(*value),
                            Unit::Percent => style.size.height = Dimension::Percent(*value),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        NodeType::Text(_) => {}
    }

    if dom_node.children.is_empty() {
        match tree.new_leaf(style) {
            Ok(leaf_id) => return Some(leaf_id),
            Err(_) => return None,
        }
    }

    let mut children_ids = Vec::new();
    for child_render_node in &render_node.children {
        match generate_node(tree, render_tree, child_render_node) {
            Some(child_id) => children_ids.push(child_id),
            None => continue,
        }
    }

    match tree.new_with_children(style, &children_ids) {
        Ok(node_id) => Some(node_id),
        Err(_) => None,
    }
}
