pub mod commands;

use std::ops::AddAssign;
use pangocairo::pango::FontDescription;
use rand::Rng;
use crate::layering::layer::LayerList;
use crate::layouter::ElementContext;
use crate::painter::commands::border::{Border, BorderStyle};
use crate::painter::commands::brush::Brush;
use crate::painter::commands::color::Color;
use crate::painter::commands::rectangle::Rectangle;
use crate::painter::commands::PaintCommand;
use crate::common::get_image_store;
use crate::painter::commands::text::Text;
use crate::tiler::Tile;

pub struct Painter {}

impl Painter {
    // Generate paint commands for the given tile
    pub(crate) fn paint(tile: &Tile, layer_list: &LayerList) -> Vec<PaintCommand> {
        let mut commands = Vec::new();

        for tile_element in &tile.elements {
            let Some(layout_element) = layer_list.layout_tree.get_node_by_id(tile_element.id) else {
                continue;
            };
            // let Some(render_element) = layer_list.layout_tree.render_tree.get_node_by_id(layout_element.render_node_id) else {
            //     continue;
            // };
            // let Some(node) = layer_list.layout_tree.render_tree.doc.get_node_by_id(layout_element.dom_node_id) else {
            //     continue;
            // };

            match &layout_element.context {
                ElementContext::Text(ctx) => {
                    let brush = Brush::solid(Color::BLACK);
                    let t = Text::new(
                        layout_element.box_model.content_box(),
                        &ctx.text,
                        FontDescription::from_string(&format!("{} {}", ctx.font_family, ctx.font_size)),
                        ctx.font_size,
                        brush
                    );
                    commands.push(PaintCommand::text(t));

                    let border = Border::new(1.0, BorderStyle::Solid, Brush::Solid(Color::RED));
                    let r = Rectangle::new(layout_element.box_model.border_box()).with_border(border);
                    commands.push(PaintCommand::rectangle(r));

                }
                ElementContext::Image(image_ctx) => {
                    let binding = get_image_store();
                    let image_store = binding.read().expect("Failed to get image store");
                    let image = image_store.get(image_ctx.image_id).unwrap();

                    let brush = Brush::image(image.data.clone(), image.width as u32, image.height as u32);
                    let border = Border::new(3.0, BorderStyle::Dashed, Brush::Solid(Color::GREEN));
                    let r = Rectangle::new(layout_element.box_model.border_box()).with_background(brush).with_border(border);
                    commands.push(PaintCommand::rectangle(r));
                }
                ElementContext::None => {
                    let border = Border::new(1.0, BorderStyle::Dotted, Brush::Solid(Color::BLUE));
                    let r = Rectangle::new(layout_element.box_model.border_box()).with_border(border);
                    commands.push(PaintCommand::rectangle(r));
                }
            }
        }

        commands
    }
}
