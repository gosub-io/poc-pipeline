use gtk4::cairo::Context;
use gtk4::pango::{Glyph, GlyphString};
use pangocairo::functions::show_glyph_string;
use parley::{Layout, PositionedLayoutItem};
use crate::document::node::NodeType;
use crate::layering::layer::{LayerId, LayerList};
use crate::layouter::{LayoutElementId, LayoutElementNode};
use crate::layouter::text::{get_font_context, get_text_layout, ColorBrush};

pub fn paint_cairo(layer_list: &LayerList, cr: &Context, visible_layer_list: Vec<bool>, wireframed: bool, hover: Option<LayoutElementId>) {
    // white background
    cr.set_source_rgb(1.0, 1.0, 1.0);
    _ = cr.paint();

    fn draw_layer(cr: &Context, layer_list: &LayerList, layer_id: LayerId, wireframed: bool, hover: Option<LayoutElementId>) {
        fn draw_wireframe(cr: &Context, el: &LayoutElementNode) {

            // Draw margin
            let m = el.box_model.margin_box;
            cr.set_source_rgba(1.0, 0.0, 0.0, 1.0);
            cr.rectangle(m.x, m.y, m.width, m.height);
            _ = cr.stroke();

            // Draw border
            let b = el.box_model.border_box();
            cr.set_source_rgba(48.0 / 255.0, 12.0 / 255.0, 124.0 / 255.0, 0.25);
            cr.rectangle(b.x, b.y, b.width, b.height);
            _ = cr.stroke();

            // Draw padding (blue)
            cr.set_source_rgba(173.0 / 255.0, 173.0 / 255.0, 247.0 / 255.0, 0.25);
            let p = el.box_model.padding_box();
            cr.rectangle(p.x, p.y, p.width, p.height);
            _ = cr.stroke();

            // Draw content (white fill with black stroke)
            let c = el.box_model.content_box();
            cr.set_source_rgba(173.0 / 255.0, 244.0 / 255.0, 247.0 / 255.0, 0.25);
            cr.rectangle(c.x, c.y, c.width, c.height);
            _ = cr.stroke();

            cr.rectangle(m.x, m.y, m.width, m.height);
            cr.set_source_rgba(1.0, 0.0, 0.0, 0.25);
            _ = cr.stroke();

        }
        fn draw_debug_boxmodel(cr: &Context, el: &LayoutElementNode) {
            // Draw margin
            let m = el.box_model.margin_box;
            cr.set_source_rgba(243.0 / 255.0, 243.0 / 255.0, 173.0 / 255.0, 0.25);
            cr.rectangle(m.x, m.y, m.width, m.height);
            _ = cr.fill();

            // Draw border
            let b = el.box_model.border_box();
            cr.set_source_rgba(48.0 / 255.0, 12.0 / 255.0, 124.0 / 255.0, 0.25);
            cr.rectangle(b.x, b.y, b.width, b.height);
            _ = cr.fill();

            // Draw padding (blue)
            cr.set_source_rgba(173.0 / 255.0, 173.0 / 255.0, 247.0 / 255.0, 0.25);
            let p = el.box_model.padding_box();
            cr.rectangle(p.x, p.y, p.width, p.height);
            _ = cr.fill();

            // Draw content (white fill with black stroke)
            let c = el.box_model.content_box();
            cr.set_source_rgba(173.0 / 255.0, 244.0 / 255.0, 247.0 / 255.0, 0.25);
            cr.rectangle(c.x, c.y, c.width, c.height);
            _ = cr.fill();

            cr.rectangle(m.x, m.y, m.width, m.height);
            cr.set_source_rgba(1.0, 0.0, 0.0, 0.25);
            _ = cr.stroke();
        }

        fn draw_paint(cr: &Context, layer_list: &LayerList, el: &LayoutElementNode) {
            let Some(node) = layer_list.layout_tree.render_tree.doc.get_node_by_id(el.dom_node_id) else {
                return;
            };

            // If we have a text element, we already have text layout which we can render the glyphsy

            match &node.node_type {
                NodeType::Element(ref el_data) => {
                    // if let Some(ref style) = el_data.computed_style {
                    //     if let Some(ref bg_color) = style.background_color {
                    //         cr.set_source_rgba(bg_color.r, bg_color.g, bg_color.b, bg_color.a);
                    //         cr.rectangle(el.box_model.content_box().x, el.box_model.content_box().y, el.box_model.content_box().width, el.box_model.content_box().height);
                    //         _ = cr.fill();
                    //     }
                    // }
                }
                NodeType::Text(text) => {
                    let layout = get_text_layout(text, "Arial", 12.0);

                    rasterize_text_layout(cr, layout);
                }
                _ => {}
            }


            cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
            cr.rectangle(el.box_model.content_box().x, el.box_model.content_box().y, el.box_model.content_box().width, el.box_model.content_box().height);
            _ = cr.fill();
        }

        let binding = layer_list.layers.borrow();
        let Some(layer) = binding.get(layer_id) else {
            return;
        };

        for el_node_id in &layer.elements {
            let el = layer_list.layout_tree.get_node_by_id(*el_node_id).unwrap();

            // Skip this node if it's not the hovernode we need to display
            if hover.is_some() && hover.unwrap() != el.id {
                continue;
            }

            if wireframed {
                draw_wireframe(cr, el);
            } else {
                draw_debug_boxmodel(cr, el);
            }
        }
    }

    for (layer_id, visible) in visible_layer_list.iter().enumerate() {
        if *visible {
            draw_layer(cr, &layer_list, layer_id as LayerId, wireframed, hover);
        }
    }
}



fn raster_text_layout(cr: &Context, layout: Layout<ColorBrush>, offset_x: f32, offset_y: f32) {
    let font_ctx = get_font_context();

    // The layouter has cut the text into different lines for us.
    for line in layout.lines() {
        // Each item is either a run of glyps or an inline box.
        for item in line.items() {
            match item {
                PositionedLayoutItem::GlyphRun(glyph_run) => {
                    let grun = glyph_run.run();

                    let gs = GlyphString::new();

                    show_glyph_string(cr, &grun.font, &grun.glyphs);


                    // Find the font that is accompanied by this glyph run, or generate it if it does not exist yet.
                    let font_id = grun.font().data.id();
                    let font_face = create_memory_font_face(fctx.ft_lib.clone(), grun.font());
                    cr.set_font_face(font_face);

                    cr.set_font_size(glyph_run.run().font_size() as f64);

                    // Render per glyph
                    cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);

                    // Glyphs are already positioned by the layouter. However, we must take into account
                    // that our offset is not 0,0 but offset_x, offset_y.
                    let glyphs: Vec<Glyph> = glyph_run
                        .positioned_glyphs()
                        .map(|g| Glyph::new(g.id as u64, offset_x as f64 + g.x as f64, offset_y as f64 + g.y as f64))
                        .collect();

                    // We can show the set of glyphs as a whole now
                    cr.show_glyphs(glyphs.as_slice()).unwrap();
                }
                PositionedLayoutItem::InlineBox(inline_box) => {
                    cr.rectangle(
                        (offset_x + inline_box.x) as f64,
                        (offset_y + inline_box.y) as f64,
                        inline_box.width as f64,
                        inline_box.height as f64,
                    );
                    cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
                    let _ = cr.stroke();

                    cr.rectangle(
                        (offset_x + inline_box.x) as f64,
                        (offset_y + inline_box.y) as f64,
                        inline_box.width as f64,
                        inline_box.height as f64,
                    );
                    cr.set_source_rgba(0.0, 0.0, 1.0, 0.25);
                    let _ = cr.fill();
                }
            };
        }
    }
