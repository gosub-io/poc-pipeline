use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use gtk4::pango;
use taffy::prelude::*;
use taffy::NodeId as TaffyNodeId;
use crate::rendertree_builder::{RenderTree, RenderNodeId};
use crate::common::document::node::{NodeType, NodeId as DomNodeId};
use crate::common::document::style::{StyleProperty, StyleValue, Unit};
use crate::common::get_image_store;
use crate::common::image::ImageId;
use crate::layouter::{LayoutElementNode, LayoutTree, LayoutElementId, CanLayout, ElementContext, box_model, ElementContextText, ElementContextImage};
use crate::layouter::css_taffy_converter::CssTaffyConverter;
use crate::layouter::text::pango::get_text_layout;

const DEFAULT_FONT_SIZE: f64 = 16.0;
const DEFAULT_FONT_FAMILY: &str = "Sans";

pub struct TaffyLayouter {
    /// Generated taffy tree
    tree: TaffyTree<TaffyContext>,
    /// Root id of the taffy tree
    root_id: TaffyNodeId,
    /// Mapping of layout element id to taffy node id
    layout_taffy_mapping: HashMap<LayoutElementId, TaffyNodeId>,
}

/// @TODO: we have taffy context structures, which contains information for layouting with taffy. But
/// the information is probably also needed in other parts. For this we also have the ElementContext
/// in the layout node. We probably want to merge these two structures into one, so we don't have to
/// duplicate the information.
#[derive(Clone, Debug)]
pub enum TaffyContext {
    Text(ElementContextText),
    Image(ElementContextImage),
}

impl TaffyContext {
    fn text(font_family: &str, font_size: f64, font_weight: usize, text: &str) -> TaffyContext {
        TaffyContext::Text(ElementContextText{
            font_family: font_family.to_string(),
            font_size,
            font_weight,
            text: text.to_string(),
        })
    }

    fn image(src: &str, image_id: ImageId, dimension: crate::common::geo::Dimension) -> TaffyContext {
        TaffyContext::Image(ElementContextImage{
            src: src.to_string(),
            image_id,
            dimension,
        })
    }
}

impl TaffyLayouter {
    pub fn new() -> Self {
        Self {
            tree: TaffyTree::new(),
            root_id: TaffyNodeId::new(0),
            layout_taffy_mapping: HashMap::new(),
        }
    }
}

impl CanLayout for TaffyLayouter {
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
                    let font_size = text_ctx.font_size;
                    let font_weight = text_ctx.font_weight;
                    let font_family = text_ctx.font_family.as_str();
                    let text = text_ctx.text.as_str();

                    dbg!(&text_ctx);

                    let layout = get_text_layout(text, font_family, font_size, font_weight, v_as.width.unwrap_or(100.0) as f64);
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
        layout_tree.root_dimension = crate::common::geo::Dimension::new(w as f64, h as f64);

        layout_tree
    }
}

impl TaffyLayouter {
    // Populate the layout tree with the boxmodels that we now can generate
    fn populate_boxmodel(&self, layout_tree: &mut LayoutTree, layout_node_id: LayoutElementId, offset: crate::common::geo::Coordinate) {
        let taffy_node_id = self.layout_taffy_mapping.get(&layout_node_id).unwrap();
        let layout = self.tree.layout(*taffy_node_id).unwrap().clone();

        let el = layout_tree.get_node_by_id_mut(layout_node_id).unwrap();
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
            root_dimension: crate::common::geo::Dimension::ZERO,
        };

        let ids = {
            let layout_element_id = {
                let el = self.generate_node(&mut layout_tree, root_id).unwrap();
                el.id
            };

            let taffy_node_id = self.layout_taffy_mapping.get(&layout_element_id).unwrap().clone();
            (taffy_node_id, layout_element_id)
        };

        self.root_id = ids.0;
        layout_tree.root_id = ids.1;

        Some(layout_tree)
    }

    fn generate_node<'a>(&mut self, layout_tree: &'a mut LayoutTree, render_node_id: RenderNodeId) -> Option<&'a LayoutElementNode> {
        // Default taffy style
        let mut taffy_style = Style {
            display: Display::Block,
            ..Default::default()
        };

        // Additional taffy context based on the DOM node.
        let mut taffy_context = None;

        // Find the DOM node in the DOM document that is wrapped in the render tree
        let dom_node_id = DomNodeId::from(render_node_id);   // DOM node IDs and render node IDs are interchangeable
        let Some(dom_node) = layout_tree.render_tree.doc.get_node_by_id(dom_node_id) else {
            return None;
        };

        match &dom_node.node_type {
            NodeType::Element(data) => {
                let conv = CssTaffyConverter::new(&data.styles);
                conv.convert(&mut taffy_style);

                // Check if element type is an image
                if data.tag_name.eq_ignore_ascii_case("img") {
                    let src = data.get_attribute("src").unwrap();
                    println!("Loading image: {}", src);

                    let store = get_image_store();
                    let image_id = store.read().unwrap().store_from_path(src.as_str());

                    let image = store.read().unwrap().get(image_id).unwrap();
                    let dimension = crate::common::geo::Dimension::new(image.width as f64, image.height as f64);

                    taffy_context = Some(TaffyContext::image(src.as_str(), image_id, dimension));
                }
            }
            NodeType::Text(text, node_style) => {
                // Default font
                let mut font_size = DEFAULT_FONT_SIZE;
                let mut font_family = DEFAULT_FONT_FAMILY.to_string();

                match node_style.get_property(StyleProperty::FontSize) {
                    Some(StyleValue::Unit(value, unit)) => {
                        match unit {
                            Unit::Px => font_size = *value as f64,
                            Unit::Em => panic!("Don't know how to deal with em units for fonts"),
                            Unit::Rem => panic!("Don't know how to deal with rem units for fonts"),
                            _ => panic!("Incorrect font-size property unit"),
                        }
                    }
                    _ => {},
                }

                match node_style.get_property(StyleProperty::FontFamily) {
                    Some(StyleValue::Keyword(value)) => font_family = value.clone(),
                    _ => {},
                }

                let font_weight = match node_style.get_property(StyleProperty::FontWeight) {
                    Some(StyleValue::Number(value)) => *value,
                    _ => 400.0,
                };

                taffy_context = Some(TaffyContext::text(
                    font_family.as_str(),
                    font_size,
                    font_weight as usize,
                    text,
                ));
            }
            NodeType::Comment(_) => {}
        }

        if dom_node.children.is_empty() {
            let element_context = to_element_context(taffy_context.as_ref());

            let result = match taffy_context {
                Some(taffy_context) => self.tree.new_leaf_with_context(taffy_style, taffy_context),
                None => self.tree.new_leaf(taffy_style),
            };

            match result {
                Ok(leaf_id) => {
                    let el = LayoutElementNode {
                        id: layout_tree.next_node_id(),
                        dom_node_id,
                        render_node_id,
                        // taffy_node_id: leaf_id,
                        box_model: box_model::BoxModel::ZERO,
                        children: vec![],
                        context: element_context,
                    };

                    self.layout_taffy_mapping.insert(el.id, leaf_id);

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
            let result = {
                let res = self.generate_node(layout_tree, *child_render_node_id);
                match res {
                    Some(el) => {
                        let taffy_node_id = self.layout_taffy_mapping.get(&el.id).unwrap().clone();
                        Some((taffy_node_id, el.id))
                    },
                    None => None,
                }
            };

            match result {
                Some(ids) => {
                    children_taffy_ids.push(ids.0);
                    children_el_ids.push(ids.1);
                },
                None => continue,
            }
        }

        match self.tree.new_with_children(taffy_style, &children_taffy_ids) {
            Ok(leaf_id) => {
                let element_context = to_element_context(taffy_context.as_ref());
                let el = LayoutElementNode {
                    id: layout_tree.next_node_id(),
                    dom_node_id,
                    render_node_id,
                    box_model: box_model::BoxModel::ZERO,
                    children: children_el_ids,
                    context: element_context,
                };

                self.layout_taffy_mapping.insert(el.id, leaf_id);

                let id = el.id;
                layout_tree.arena.insert(id, el);
                layout_tree.arena.get(&id)

            }
            Err(_) => None,
        }
    }

}

/// Convert a taffy context to an element context. Optiomally, these two structures should be merged
/// and only ElementContext should be used.
fn to_element_context(taffy_context: Option<&TaffyContext>) -> ElementContext {
    match taffy_context {
        Some(TaffyContext::Text(text_ctx)) => ElementContext::text(
            text_ctx.font_family.as_str(),
            text_ctx.font_size,
            text_ctx.font_weight,
            text_ctx.text.as_str()
        ),
        Some(TaffyContext::Image(image_ctx)) => ElementContext::image(
            image_ctx.src.as_str(),
            image_ctx.image_id,
            image_ctx.dimension.clone(),
        ),
        None => ElementContext::None,
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
pub fn taffy_layout_to_boxmodel(layout: &Layout, offset: crate::common::geo::Coordinate) -> box_model::BoxModel {
    box_model::BoxModel {
        margin_box: crate::common::geo::Rect {
            x: offset.x + layout.location.x as f64,
            y: offset.y + layout.location.y as f64,
            width: layout.size.width as f64 + layout.margin.left as f64 + layout.margin.right as f64,
            height: layout.size.height as f64 + layout.margin.top as f64 + layout.margin.bottom as f64,
        },
        padding: box_model::Edges {
            top: layout.padding.top as f64,
            right: layout.padding.right as f64,
            bottom: layout.padding.bottom as f64,
            left: layout.padding.left as f64,
        },
        border: box_model::Edges {
            top: layout.border.top as f64,
            right: layout.border.right as f64,
            bottom: layout.border.bottom as f64,
            left: layout.border.left as f64,
        },
        margin: box_model::Edges {
            top: layout.margin.top as f64,
            right: layout.margin.right as f64,
            bottom: layout.margin.bottom as f64,
            left: layout.margin.left as f64,
        }
    }
}
