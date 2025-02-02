use crate::render_tree::{RenderNode, RenderTree};
use taffy::prelude::*;
use crate::document::node::NodeType;
use crate::document::style::{StyleValue, Unit};
use crate::layouter::text::measure_text_height;
use crate::layouter::ViewportSize;

pub fn generate_taffy_tree(render_tree: &RenderTree, viewport: ViewportSize) -> TaffyTree {
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
            // --- Width and Height styles ---
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

            // --- Margin ---
            if let Some(margin_block_start) = data.get_style("margin-block-start") {
                match margin_block_start {
                    StyleValue::Unit(value, unit) => {
                        match unit {
                            Unit::Px => style.margin.top = LengthPercentageAuto::Length(*value),
                            Unit::Percent => style.margin.top = LengthPercentageAuto::Percent(*value),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            if let Some(margin_block_end) = data.get_style("margin-block-end") {
                match margin_block_end {
                    StyleValue::Unit(value, unit) => {
                        match unit {
                            Unit::Px => style.margin.bottom = LengthPercentageAuto::Length(*value),
                            Unit::Percent => style.margin.bottom = LengthPercentageAuto::Percent(*value),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            if let Some(margin_inline_start) = data.get_style("margin-inline-start") {
                match margin_inline_start {
                    StyleValue::Unit(value, unit) => {
                        match unit {
                            Unit::Px => style.margin.left = LengthPercentageAuto::Length(*value),
                            Unit::Percent => style.margin.left = LengthPercentageAuto::Percent(*value),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            if let Some(margin_inline_end) = data.get_style("margin-inline-end") {
                match margin_inline_end {
                    StyleValue::Unit(value, unit) => {
                        match unit {
                            Unit::Px => style.margin.right = LengthPercentageAuto::Length(*value),
                            Unit::Percent => style.margin.right = LengthPercentageAuto::Percent(*value),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            // --- Padding ---
            if let Some(padding_block_start) = data.get_style("padding-block-start") {
                match padding_block_start {
                    StyleValue::Unit(value, unit) => {
                        match unit {
                            Unit::Px => style.padding.top = LengthPercentage::Length(*value),
                            Unit::Percent => style.padding.top = LengthPercentage::Percent(*value),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            if let Some(padding_block_end) = data.get_style("padding-block-end") {
                match padding_block_end {
                    StyleValue::Unit(value, unit) => {
                        match unit {
                            Unit::Px => style.padding.bottom = LengthPercentage::Length(*value),
                            Unit::Percent => style.padding.bottom = LengthPercentage::Percent(*value),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            if let Some(padding_inline_start) = data.get_style("padding-inline-start") {
                match padding_inline_start {
                    StyleValue::Unit(value, unit) => {
                        match unit {
                            Unit::Px => style.padding.left = LengthPercentage::Length(*value),
                            Unit::Percent => style.padding.left = LengthPercentage::Percent(*value),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            if let Some(padding_inline_end) = data.get_style("padding-inline-end") {
                match padding_inline_end {
                    StyleValue::Unit(value, unit) => {
                        match unit {
                            Unit::Px => style.padding.right = LengthPercentage::Length(*value),
                            Unit::Percent => style.padding.right = LengthPercentage::Percent(*value),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

        }
        NodeType::Text(text) => {
            let font_size = 16.0;
            let line_height = 18.0;
            style.size.height = Dimension::Length(
                measure_text_height(text, font_size, line_height) as f32
            );

        }
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
