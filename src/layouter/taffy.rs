use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use gtk4::pango;
use crate::rendertree_builder::{RenderTree, RenderNodeId};
use crate::common::document::node::{NodeType, NodeId as DomNodeId};
use crate::common::document::style::{StyleProperty, StyleValue, Unit};
use crate::layouter::{boxmodel as BoxModel, LayoutElementNode, LayoutTree, TaffyStruct, TaffyNodeId, LayoutElementId, CanLayout};
use taffy::prelude::*;
use crate::layouter::text::pango::get_text_layout;

// Taffy context for a text node
#[derive(Clone, Debug)]
pub struct TextContext {
    text: String,
    family: String,
    size: f32,
}

// Taffy context for an image node (needed so we can define the width/height of the image)
#[derive(Clone, Debug)]
pub struct ImageContext {
    src: String,
}

#[derive(Clone, Debug)]
pub enum TaffyContext {
    Text(TextContext),
    Image(ImageContext),
}


pub struct TaffyLayouter {
    tree: TaffyTree<TaffyContext>,
    root_id: TaffyNodeId,
}

impl TaffyLayouter {
    pub fn new() -> Self {
        Self {
            tree: TaffyTree::new(),
            root_id: TaffyNodeId::new(0),
        }
    }
}

impl CanLayout for TaffyLayouter {
    /// Generates a layout tree based on taffy. Note that the layout tree current holds taffy information (like styles)
    /// that we probably want to convert to our own style system. We already do this with the taffy layout through the
    /// BoxModel structure.
    fn layout(&mut self, render_tree: RenderTree, viewport: crate::common::geo::Dimension) -> LayoutTree {
        let root_id = render_tree.root_id.unwrap();
        let Some(mut layout_tree) = self.generate_tree(render_tree, root_id) else {
            panic!("Failed to generate root node render tree");
        };

        // Compute the layout based on the viewport
        let size = Size {
            width: AvailableSpace::Definite(viewport.width as f32),
            height: AvailableSpace::Definite(viewport.height as f32),
        };

        self.tree.compute_layout_with_measure(self.root_id, size, |v_kd, v_as, v_ni, v_nc, v_s| {
            match v_nc {
                Some(TaffyContext::Text(text_ctx)) => {
                    let font_size = text_ctx.size;
                    let font_family = text_ctx.family.as_str();
                    let text = text_ctx.text.as_str();

                    let layout = get_text_layout(text, font_family, font_size as f64, v_as.width.unwrap() as f64);
                    match layout {
                        Ok(layout) => {
                            // @TODO: Somehow, layout.width() and layout.height() do not seem to work anymore
                            let (_, logical_rect) = layout.extents();

                            Size {
                                width: logical_rect.width() as f32 / pango::SCALE as f32,
                                height: logical_rect.height() as f32 / pango::SCALE as f32,
                            }
                        },
                        Err(_) => Size::ZERO
                    }
                },
                _ => Size::ZERO
            }
        }).unwrap();

        // Generate box model for the whole layout tree
        let root_id = layout_tree.root_id;
        self.populate_boxmodel(&mut layout_tree, root_id, crate::common::geo::Coordinate::ZERO);

        // get dimension of the root node
        let root = layout_tree.get_node_by_id(root_id).unwrap();
        let w = root.box_model.margin_box.width as f32;
        let h = root.box_model.margin_box.height as f32;
        layout_tree.root_width = w;
        layout_tree.root_height = h;

        layout_tree
    }
}

impl TaffyLayouter {
    // Populate the layout tree with the boxmodels that we now can generate
    fn populate_boxmodel(&self, layout_tree: &mut LayoutTree, node_id: LayoutElementId, offset: crate::common::geo::Coordinate) {
        let el = layout_tree.get_node_by_id(node_id).unwrap();
        let layout = self.tree.layout(el.taffy_node_id).unwrap().clone();

        let el = layout_tree.get_node_by_id_mut(node_id).unwrap();
        el.box_model = taffy_layout_to_boxmodel(&layout, offset);
        let child_ids = el.children.clone();

        for child_id in child_ids {
            self.populate_boxmodel(layout_tree, child_id, crate::common::geo::Coordinate::new(
                offset.x + layout.location.x as f64 + layout.margin.left as f64,
                offset.y + layout.location.y as f64 + layout.margin.top as f64
            ));
        }
    }

    fn generate_tree(&mut self, render_tree: RenderTree, root_id: RenderNodeId) -> Option<LayoutTree> {
        self.tree = TaffyTree::new();
        self.root_id = TaffyNodeId::new(0); // Will be filled in later

        let mut layout_tree = LayoutTree {
            render_tree,
            arena: HashMap::new(),
            root_id: LayoutElementId::new(0), // Will be filled in later
            next_node_id: Arc::new(RwLock::new(LayoutElementId::new(0))),
            root_width: 0.0,
            root_height: 0.0,
        };

        let ids = {
            let temp_el = self.generate_node(&mut layout_tree, root_id).unwrap();
            (temp_el.taffy_node_id, temp_el.id)
        };

        self.root_id = ids.0;
        layout_tree.root_id = ids.1;

        Some(layout_tree)
    }

    fn generate_node(&mut self, layout_tree: &mut LayoutTree, render_node_id: RenderNodeId) -> Option<&LayoutElementNode> {
        let mut style = Style {
            display: Display::Block,
            ..Default::default()
        };

        // Context to set for text nodes
        let mut taffy_context = None;

        // Find the DOM node in the DOM document that is wrapped in the render tree
        let dom_node_id = DomNodeId::from(render_node_id);   // DOM node IDs and render node IDs are interchangeable
        let Some(dom_node) = layout_tree.render_tree.doc.get_node_by_id(dom_node_id) else {
            return None;
        };

        match &dom_node.node_type {
            NodeType::Element(data) => {
                // --- Width and Height styles ---
                if let Some(width) = data.get_style(StyleProperty::Width) {
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
                if let Some(height) = data.get_style(StyleProperty::Height) {
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
                if let Some(margin_block_start) = data.get_style(StyleProperty::MarginTop) {
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
                if let Some(margin_block_end) = data.get_style(StyleProperty::MarginBottom) {
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
                if let Some(margin_inline_start) = data.get_style(StyleProperty::MarginLeft) {
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
                if let Some(margin_inline_end) = data.get_style(StyleProperty::MarginRight) {
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
                if let Some(padding_block_start) = data.get_style(StyleProperty::PaddingTop) {
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
                if let Some(padding_block_end) = data.get_style(StyleProperty::PaddingBottom) {
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
                if let Some(padding_inline_start) = data.get_style(StyleProperty::PaddingLeft) {
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
                if let Some(padding_inline_end) = data.get_style(StyleProperty::PaddingRight) {
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
                // --- Border ---
                if let Some(border_top_width) = data.get_style(StyleProperty::BorderTopWidth) {
                    match border_top_width {
                        StyleValue::Unit(value, unit) => {
                            match unit {
                                Unit::Px => style.border.top = LengthPercentage::Length(*value),
                                Unit::Percent => style.border.top = LengthPercentage::Percent(*value),
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                if let Some(border_bottom_width) = data.get_style(StyleProperty::BorderBottomWidth) {
                    match border_bottom_width {
                        StyleValue::Unit(value, unit) => {
                            match unit {
                                Unit::Px => style.border.bottom = LengthPercentage::Length(*value),
                                Unit::Percent => style.border.bottom = LengthPercentage::Percent(*value),
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                if let Some(border_left_width) = data.get_style(StyleProperty::BorderLeftWidth) {
                    match border_left_width {
                        StyleValue::Unit(value, unit) => {
                            match unit {
                                Unit::Px => style.border.left = LengthPercentage::Length(*value),
                                Unit::Percent => style.border.left = LengthPercentage::Percent(*value),
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                if let Some(border_right_width) = data.get_style(StyleProperty::BorderRightWidth) {
                    match border_right_width {
                        StyleValue::Unit(value, unit) => {
                            match unit {
                                Unit::Px => style.border.right = LengthPercentage::Length(*value),
                                Unit::Percent => style.border.right = LengthPercentage::Percent(*value),
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
            NodeType::Text(text, style) => {
                // @TODO: make sure font here is from the style property list
                let mut font_size = 16.0;
                let mut font_family = "Arial".to_string();

                match style.get_property(StyleProperty::FontSize) {
                    Some(StyleValue::Unit(value, unit)) => {
                        match unit {
                            Unit::Px => font_size = *value,
                            Unit::Em => panic!("Don't know how to deal with em units for fonts"),
                            Unit::Rem => panic!("Don't know how to deal with rem units for fonts"),
                            _ => panic!("Incorrect font-size property unit"),
                        }
                    }
                    _ => {},
                }

                match style.get_property(StyleProperty::FontFamily) {
                    Some(StyleValue::Keyword(value)) => font_family = value.clone(),
                    _ => {},
                }

                taffy_context = Some(TaffyContext::Text(
                    TextContext {
                        text: text.clone(),
                        family: font_family,
                        size: font_size,
                    }
                ));
            }
        }

        if dom_node.children.is_empty() {
            let result = match taffy_context {
                Some(taffy_context) => self.tree.new_leaf_with_context(style, taffy_context),
                None => self.tree.new_leaf(style),
            };

            match result {
                Ok(leaf_id) => {
                    let el = LayoutElementNode {
                        id: layout_tree.next_node_id(),
                        dom_node_id,
                        render_node_id,
                        taffy_node_id: leaf_id,
                        box_model: BoxModel::BoxModel::ZERO,
                        children: vec![],
                        // context: LayoutContext::None,
                        // render_context: RenderContext::None,
                    };

                    let id = el.id;
                    layout_tree.arena.insert(id, el);
                    return layout_tree.arena.get(&id);
                },
                Err(_) => {},
            }

            return None
        }

        let mut children_taffy_ids = Vec::new();
        let mut children_el_ids = Vec::new();

        let render_node = layout_tree.render_tree.get_node_by_id(render_node_id).unwrap();
        let children = render_node.children.clone();

        for child_render_node_id in &children {
            match self.generate_node(layout_tree, *child_render_node_id) {
                Some(el) => {
                    children_taffy_ids.push(el.taffy_node_id);
                    children_el_ids.push(el.id);
                },
                None => continue,
            }
        }

        match self.tree.new_with_children(style, &children_taffy_ids) {
            Ok(leaf_id) => {
                let el = LayoutElementNode {
                    id: layout_tree.next_node_id(),
                    dom_node_id,
                    render_node_id,
                    taffy_node_id: leaf_id,
                    box_model: BoxModel::BoxModel::ZERO,
                    children: children_el_ids,
                    // context: LayoutContext::None,
                    // render_context: RenderContext::None,
                };

                let id = el.id;
                layout_tree.arena.insert(id, el);
                layout_tree.arena.get(&id)
            }
            Err(_) => None,
        }
    }

}

// Returns true if there is a margin on the rect (basically, if the rect is non-zero)
fn has_margin(src: Rect<LengthPercentageAuto>) -> bool {
    let is_zero = (src.top == LengthPercentageAuto::Length(0.0) || src.top == LengthPercentageAuto::Percent(0.0)) &&
    (src.right == LengthPercentageAuto::Length(0.0) || src.right == LengthPercentageAuto::Percent(0.0)) &&
    (src.bottom == LengthPercentageAuto::Length(0.0) || src.bottom == LengthPercentageAuto::Percent(0.0)) &&
    (src.left == LengthPercentageAuto::Length(0.0) || src.left == LengthPercentageAuto::Percent(0.0));

    !is_zero
}

/// Converts a taffy layout to our own BoxModel structure
pub fn taffy_layout_to_boxmodel(layout: &Layout, offset: crate::common::geo::Coordinate) -> BoxModel::BoxModel {
    BoxModel::BoxModel {
        margin_box: crate::common::geo::Rect {
            x: offset.x + layout.location.x as f64,
            y: offset.y + layout.location.y as f64,
            width: layout.size.width as f64 + layout.margin.left as f64 + layout.margin.right as f64,
            height: layout.size.height as f64 + layout.margin.top as f64 + layout.margin.bottom as f64,
        },
        padding: BoxModel::Edges {
            top: layout.padding.top as f64,
            right: layout.padding.right as f64,
            bottom: layout.padding.bottom as f64,
            left: layout.padding.left as f64,
        },
        border: BoxModel::Edges {
            top: layout.border.top as f64,
            right: layout.border.right as f64,
            bottom: layout.border.bottom as f64,
            left: layout.border.left as f64,
        },
        margin: BoxModel::Edges {
            top: layout.margin.top as f64,
            right: layout.margin.right as f64,
            bottom: layout.margin.bottom as f64,
            left: layout.margin.left as f64,
        }
    }
}
