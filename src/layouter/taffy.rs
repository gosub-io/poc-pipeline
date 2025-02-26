use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use gtk4::pango;
use taffy::prelude::*;
use taffy::NodeId as TaffyNodeId;
use crate::rendertree_builder::{RenderTree, RenderNodeId};
use crate::common::document::node::{NodeType, NodeId as DomNodeId, ElementData};
use crate::common::document::style::{FontWeight, StyleProperty, StyleValue, Unit};
use crate::common::{geo, get_image_store};
use crate::common::geo::Coordinate;
use crate::common::image::ImageId;
use crate::layouter::{LayoutElementNode, LayoutTree, LayoutElementId, CanLayout, ElementContext, box_model, ElementContextText, ElementContextImage};
use crate::layouter::css_taffy_converter::CssTaffyConverter;
use crate::layouter::text::pango::get_text_layout;
use crate::rendertree_builder::tree::RenderNode;

const DEFAULT_FONT_SIZE: f64 = 16.0;
const DEFAULT_FONT_FAMILY: &str = "Sans";

/// Layouter structure that uses taffy as layout engine
pub struct TaffyLayouter {
    /// Generated taffy tree
    tree: TaffyTree<TaffyContext>,
    /// Root id of the taffy tree
    root_id: TaffyNodeId,
    /// Mapping of layout element id to taffy node id
    layout_taffy_mapping: HashMap<LayoutElementId, TaffyNodeId>,
}

/// Context structures to pass to taffy measure functions so we can calculate the size of the text or image.
#[derive(Clone, Debug)]
pub enum TaffyContext {
    Text(ElementContextText),
    Image(ElementContextImage),
}

impl TaffyContext {
    fn text(font_family: &str, font_size: f64, font_weight: usize, line_height: f64, text: &str, node_id: DomNodeId, text_offset: Coordinate) -> TaffyContext {
        TaffyContext::Text(ElementContextText{
            node_id,
            font_family: font_family.to_string(),
            font_size,
            font_weight,
            line_height,
            text: text.to_string(),
            text_offset,
        })
    }

    fn image(src: &str, image_id: ImageId, dimension: geo::Dimension, node_id: DomNodeId) -> TaffyContext {
        TaffyContext::Image(ElementContextImage{
            node_id,
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

    pub fn print_tree(&mut self) {
        self.tree.print_tree(self.root_id);
    }
}

impl CanLayout for TaffyLayouter {
    fn layout(&mut self, render_tree: RenderTree, viewport: Option<geo::Dimension>) -> LayoutTree {
        let root_id = render_tree.root_id.unwrap();
        let Some(mut layout_tree) = self.generate_tree(render_tree, root_id) else {
            panic!("Failed to generate root node render tree");
        };

        // // Compute the layout based on the viewport
        let size = match viewport {
            Some(viewport) => Size {
                width: AvailableSpace::Definite(viewport.width as f32),
                height: AvailableSpace::Definite(viewport.height as f32),
            },
            None => Size::MAX_CONTENT,
        };

        /// Compute the layout with a measure function
        self.tree.compute_layout_with_measure(self.root_id, size, |v_kd, v_as, v_ni, v_nc, v_s| {
            match v_nc {
                // Calculate text node
                Some(TaffyContext::Text(text_ctx)) => {
                    let font_size = text_ctx.font_size;
                    let font_weight = text_ctx.font_weight;
                    let font_family = text_ctx.font_family.as_str();
                    let text = text_ctx.text.as_str();
                    let line_height = text_ctx.line_height;

                    let max_width = match v_as.width {
                        AvailableSpace::Definite(width) => width as f64,
                        AvailableSpace::MaxContent => f64::MAX,
                        AvailableSpace::MinContent => 0.0,
                    };

                    // Calculate the text layout dimensions and return it to taffy
                    let layout = get_text_layout(
                        text,
                        font_family,
                        font_size,
                        font_weight,
                        line_height,
                        max_width,
                    );
                    match layout {
                        Ok(layout) => {
                            let (_rect, logical_rect) = layout.extents();
                            let text_width = logical_rect.width() as f32 / pango::SCALE as f32;
                            let text_height = logical_rect.height() as f32 / pango::SCALE as f32;

                            Size {
                                width: text_width,
                                height: text_height,
                            }
                        },
                        Err(_) => Size::ZERO
                    }
                },
                _ => Size::ZERO
            }
        }).unwrap();

        // Since we are not interested in taffy layout after this stage in the pipeline, we convert
        // the taffy layout to a box model layout tree. This makes the rest of the pipeline
        // layout-engine agnostic.
        let root_id = layout_tree.root_id;
        self.populate_boxmodel(&mut layout_tree, root_id, Coordinate::ZERO);

        // get dimension of the root node
        let root = layout_tree.get_node_by_id(root_id).unwrap();
        let w = root.box_model.margin_box.width as f32;
        let h = root.box_model.margin_box.height as f32;
        layout_tree.root_dimension = geo::Dimension::new(w as f64, h as f64);

        layout_tree
    }
}

impl TaffyLayouter {
    // Populate the layout tree with the boxmodels that we now can generate
    fn populate_boxmodel(&self, layout_tree: &mut LayoutTree, layout_node_id: LayoutElementId, offset: Coordinate) {
        let taffy_node_id = self.layout_taffy_mapping.get(&layout_node_id).unwrap();
        let layout = self.tree.layout(*taffy_node_id).unwrap().clone();

        let el = layout_tree.get_node_by_id_mut(layout_node_id).unwrap();
        el.box_model = taffy_layout_to_boxmodel(&layout, offset);
        let child_ids = el.children.clone();

        for child_id in child_ids {
            self.populate_boxmodel(layout_tree, child_id, Coordinate::new(
                offset.x + layout.location.x as f64 + layout.margin.left as f64,
                offset.y + layout.location.y as f64 + layout.margin.top as f64
            ));
        }
    }

    /// Generate the layout tree from the render tree
    fn generate_tree(&mut self, render_tree: RenderTree, root_id: RenderNodeId) -> Option<LayoutTree> {
        self.tree = TaffyTree::new();
        self.root_id = TaffyNodeId::new(0); // Will be filled in later

        let mut layout_tree = LayoutTree {
            render_tree,
            arena: HashMap::new(),
            root_id: LayoutElementId::new(0), // Will be filled in later
            next_node_id: Arc::new(RwLock::new(LayoutElementId::new(0))),
            root_dimension: geo::Dimension::ZERO,
            rstar_tree: rstar::RTree::new(),
        };

        let Some((layout_element_root_id, taffy_root_id)) = self.generate_node(&mut layout_tree, root_id) else {
            return None;
        };

        layout_tree.root_id = layout_element_root_id;
        self.root_id = taffy_root_id;

        Some(layout_tree)
    }

    fn generate_node<'a>(&mut self, layout_tree: &'a mut LayoutTree, render_node_id: RenderNodeId) -> Option<(LayoutElementId, TaffyNodeId)> {
        // Find render node and dom node from the layout tree
        let Some(render_node) = layout_tree.render_tree.get_node_by_id(render_node_id) else {
            return None;
        };
        let Some(dom_node) = layout_tree.render_tree.doc.get_node_by_id(DomNodeId::from(render_node.node_id)) else {
            return None;
        };
        let render_node_children = render_node.children.clone();

        // Additional taffy context based on the DOM node.
        let mut taffy_context = None;
        let mut taffy_style = Style::default();

        match &dom_node.node_type {
            // Node is an element node (like a div, span, etc.)
            NodeType::Element(data) => {
                // Create the taffy style from our CSS and push it into the stack
                let conv = CssTaffyConverter::new(&data.styles);
                taffy_style = conv.convert(dom_node.node_id);

                // Check if element type is an image, if so, set the taffy context
                if data.tag_name.eq_ignore_ascii_case("img") {
                    let src = data.get_attribute("src").unwrap();
                    println!("Loading image: {}", src);

                    let store = get_image_store();
                    let image_id = store.read().unwrap().store_from_path(src.as_str());

                    let image = store.read().unwrap().get(image_id).unwrap();
                    let dimension = geo::Dimension::new(image.width as f64, image.height as f64);

                    taffy_context = Some(TaffyContext::image(src.as_str(), image_id, dimension, dom_node.node_id));
                }

                if data.tag_name.eq_ignore_ascii_case("svg") {
                    let src = "dummy";
                    let store = get_image_store();
                    let image_id = store.read().unwrap().store_from_path(src);

                    let image = store.read().unwrap().get(image_id).unwrap();
                    let dimension = geo::Dimension::new(image.width as f64, image.height as f64);

                    taffy_context = Some(TaffyContext::image(src, image_id, dimension, dom_node.node_id));
                }
            }
            NodeType::Text(text, node_style) => {
                let parent_node = match dom_node.parent_id {
                    Some(parent_id) => layout_tree.render_tree.doc.get_node_by_id(parent_id),
                    None => None,
                };
                if parent_node.is_none() {
                    return None;
                }

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
                    Some(StyleValue::FontWeight(weight)) => match weight {
                        FontWeight::Normal => 400.0,
                        FontWeight::Bold => 700.0,
                        FontWeight::Number(value) => *value as f64,
                        FontWeight::Bolder => { unimplemented!("FontWeight::Bolder is not implemented yet") },
                        FontWeight::Lighter => { unimplemented!("FontWeight::Lighter is not implemented yet") },
                    }
                    _ => 400.0,
                };

                let line_height = match node_style.get_property(StyleProperty::LineHeight) {
                    Some(StyleValue::Unit(value, unit)) => {
                        match unit {
                            Unit::Px => *value as f64,
                            Unit::Em => panic!("Don't know how to deal with em units for line-height"),
                            Unit::Rem => panic!("Don't know how to deal with rem units for line-height"),
                            _ => panic!("Incorrect line-height property unit"),
                        }
                    }
                    _ => font_size,
                };


                // Calculate vertical offset for centering based on the lineheight.
                let text_offset = Coordinate::new(0.0, ((line_height - font_size) / 2.0));

                taffy_context = Some(TaffyContext::text(
                    font_family.as_str(),
                    font_size,
                    font_weight as usize,
                    line_height,
                    text,
                    dom_node.node_id,
                    text_offset,
                ));
            }
            NodeType::Comment(_) => {
                // No need to layouting for comment nodes. In fact, they should have been removed already
                // by the render-tree building stage.
                return None;
            }
        }

        // The context will be moved to the taffy tree, so we need to convert it before that happens.
        let element_context = to_element_context(taffy_context.as_ref());

        let result = match taffy_context {
            Some(taffy_context) => self.tree.new_leaf_with_context(taffy_style.to_owned(), taffy_context),
            None => self.tree.new_leaf(taffy_style.to_owned()),
        };

        let Ok(leaf_id) = result else {
            // Could not create a leaf node in the taffy tree
            return None;
        };

        // Create the element node in our layout tree
        let mut element_node = LayoutElementNode {
            id: layout_tree.next_node_id(),
            dom_node_id: dom_node.node_id,
            render_node_id,
            box_model: box_model::BoxModel::ZERO,
            children: vec![],
            context: element_context,
        };

        for child_id in render_node_children {
            if let Some((child_id, taffy_id)) = self.generate_node(layout_tree, child_id) {
                // Add child to taffy leaf
                self.tree.add_child(leaf_id, taffy_id);

                // Add child to layout element
                element_node.children.push(child_id);
            }
        }

        // Insert element node into our arena
        let layout_element_id = element_node.id;
        layout_tree.arena.insert(layout_element_id, element_node);

        /// Create a mapping between the layout element id and the taffy node id
        self.layout_taffy_mapping.insert(layout_element_id, leaf_id);

        Some((layout_element_id, leaf_id))
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
            text_ctx.line_height,
            text_ctx.text.as_str(),
            text_ctx.node_id,
            text_ctx.text_offset,
        ),
        Some(TaffyContext::Image(image_ctx)) => ElementContext::image(
            image_ctx.src.as_str(),
            image_ctx.image_id,
            image_ctx.dimension.clone(),
            image_ctx.node_id,
        ),
        None => ElementContext::None,
    }
}

/// Returns true if there is a margin on the rect (basically, if the rect is non-zero)
fn has_margin(src: Rect<LengthPercentageAuto>) -> bool {
    let is_zero = (src.top == LengthPercentageAuto::Length(0.0) || src.top == LengthPercentageAuto::Percent(0.0)) &&
    (src.right == LengthPercentageAuto::Length(0.0) || src.right == LengthPercentageAuto::Percent(0.0)) &&
    (src.bottom == LengthPercentageAuto::Length(0.0) || src.bottom == LengthPercentageAuto::Percent(0.0)) &&
    (src.left == LengthPercentageAuto::Length(0.0) || src.left == LengthPercentageAuto::Percent(0.0));

    !is_zero
}

/// Converts a taffy layout to our own BoxModel structure
pub fn taffy_layout_to_boxmodel(layout: &Layout, offset: Coordinate) -> box_model::BoxModel {
    box_model::BoxModel {
        margin_box: geo::Rect {
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
