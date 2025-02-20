pub mod commands;

use std::ops::AddAssign;
use std::sync::Arc;
use pangocairo::pango::FontDescription;
use rand::Rng;
use crate::common::browser_state::{get_browser_state, BrowserState, WireframeState};
use crate::common::document::node::{Node, NodeType};
use crate::common::document::style::{StyleProperty, StyleValue, Color as StyleColor};
use crate::layering::layer::LayerList;
use crate::layouter::{ElementContext, LayoutElementNode};
use crate::painter::commands::brush::Brush;
use crate::painter::commands::color::Color;
use crate::painter::commands::rectangle::{Radius, Rectangle};
use crate::painter::commands::PaintCommand;
use crate::common::get_image_store;
use crate::painter::commands::border::{Border, BorderStyle};
use crate::painter::commands::text::Text;
use crate::tiler::Tile;

pub struct Painter {
    layer_list: Arc<LayerList>,
}

impl Painter {
    pub(crate) fn new(layer_list: Arc<LayerList>) -> Painter {
        Painter {
            layer_list
        }
    }

    // Generate paint commands for the given tile
    pub(crate) fn paint(&self, tile: &Tile) -> Vec<PaintCommand> {
        let mut commands = Vec::new();

        for tile_element in &tile.elements {
            let Some(layout_element) = self.layer_list.layout_tree.get_node_by_id(tile_element.id) else {
                continue;
            };
            let Some(dom_node) = self.layer_list.layout_tree.render_tree.doc.get_node_by_id(layout_element.dom_node_id) else {
                continue;
            };

            let binding = get_browser_state();
            let state = binding.read().unwrap();

            match state.wireframed {
                WireframeState::Only => {
                    commands.extend(self.generate_wireframe_commands(&layout_element));
                }
                WireframeState::Both => {
                    commands.extend(self.generate_element_commands(&layout_element, &dom_node));
                    commands.extend(self.generate_wireframe_commands(&layout_element));
                }
                WireframeState::None => {
                    commands.extend(self.generate_element_commands(&layout_element, &dom_node));
                }
            }

        }

        commands
    }

    // Returns a brush for the color found in the given dom node
    fn get_brush(&self, node: &Node, css_prop: StyleProperty, default: Brush) -> Brush {
        let NodeType::Element(element_data) = &node.node_type else {
            log::warn!("Failed to get brush for node: {:?}", node.node_id);
            return default;
        };
        element_data.get_style(css_prop).map_or(default.clone(), |value| {
            match value {
                StyleValue::Color(css_color) => Brush::solid(convert_css_color(css_color)),
                _ => {
                    log::warn!("Failed to get brush for node: {:?}", node.node_id);
                    default.clone()
                }
            }
        })
    }

    // Returns a brush for the color found in the PARENT of the given dom node
    fn get_parent_brush(&self, node: &Node, css_prop: StyleProperty, default: Brush) -> Brush {
        let parent = match &node.parent_id {
            Some(parent_id) => self.layer_list.layout_tree.render_tree.doc.get_node_by_id(*parent_id).expect("Failed to get parent node"),
            None => {
                log::warn!("Failed to get parent brush for node: {:?}", node.node_id);
                return default
            },
        };

        self.get_brush(parent, css_prop, default)
    }

    fn generate_wireframe_commands(&self, layout_element: &LayoutElementNode) -> Vec<PaintCommand> {
        let mut commands = Vec::new();

        let border = Border::new(1.0, BorderStyle::Solid, Brush::Solid(Color::RED));
        let r = Rectangle::new(layout_element.box_model.border_box()).with_border(border);
        commands.push(PaintCommand::rectangle(r));

        commands
    }

    fn generate_element_commands(&self, layout_element: &LayoutElementNode, dom_node: &Node) -> Vec<PaintCommand> {
        let mut commands = Vec::new();

        match &layout_element.context {
            ElementContext::Text(ctx) => {
                let brush = self.get_parent_brush(dom_node, StyleProperty::Color, Brush::solid(Color::BLACK));
                // let brush = Brush::solid(Color::from_rgb8(130, 130, 130));
                let t = Text::new(
                    layout_element.box_model.content_box(),
                    &ctx.text,
                    &ctx.font_family,
                    ctx.font_size,
                    ctx.font_weight,
                    brush
                );
                commands.push(PaintCommand::text(t));

                // let border = Border::new(1.0, BorderStyle::Solid, Brush::Solid(Color::RED));
                // let r = Rectangle::new(layout_element.box_model.border_box()).with_border(border);
                // let r = Rectangle::new(layout_element.box_model.border_box()); // .with_border(border);
                // commands.push(PaintCommand::rectangle(r));
            }
            ElementContext::Image(image_ctx) => {
                let binding = get_image_store();
                let image_store = binding.read().expect("Failed to get image store");
                let image = image_store.get(image_ctx.image_id).unwrap();

                let brush = Brush::image(image.data.clone(), image.width as u32, image.height as u32);
                // let border = Border::new(3.0, BorderStyle::None, Brush::Solid(Color::GREEN));
                let r = Rectangle::new(layout_element.box_model.border_box()).with_background(brush);
                commands.push(PaintCommand::rectangle(r));
            }
            ElementContext::None => {
                let brush = self.get_brush(dom_node, StyleProperty::BackgroundColor, Brush::solid(Color::TRANSPARENT));
                // let border = Border::new(3.0, BorderStyle::None, Brush::Solid(Color::RED));
                let mut r = Rectangle::new(layout_element.box_model.border_box()).with_background(brush);

                // Get border

                // Get radius
                let radius_bottom_left = dom_node.get_style_f32(StyleProperty::BorderBottomLeftRadius);
                let radius_bottom_right = dom_node.get_style_f32(StyleProperty::BorderBottomRightRadius);
                let radius_top_left = dom_node.get_style_f32(StyleProperty::BorderTopLeftRadius);
                let radius_top_right = dom_node.get_style_f32(StyleProperty::BorderTopRightRadius);

                if (radius_bottom_left != 0.0 || radius_bottom_right != 0.0 || radius_top_left != 0.0 || radius_top_right != 0.0) {
                    r = r.with_radius_tlrb(
                        radius_top_left as Radius,
                        radius_top_right as Radius,
                        radius_bottom_right as Radius,
                        radius_bottom_left as Radius
                    );
                }

                commands.push(PaintCommand::rectangle(r));
            }
        }

        commands
    }
}

/// Converts a css style color to a paint command color
fn convert_css_color(css_color: &StyleColor) -> Color {
    log::info!("Converting css color: {:?}", css_color);
    match css_color {
        StyleColor::Named(name) => Color::from_css(name.as_str()),
        StyleColor::Rgb(r, g, b) => Color::from_rgb8(*r, *g, *b),
        StyleColor::Rgba(r, g, b, a) => Color::from_rgba8(*r, *g, *b, (*a * 255.0) as u8),
    }
}

