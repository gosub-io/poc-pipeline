use gtk4::cairo::Context;
use pangocairo::functions::{show_layout};
use crate::document::node::NodeType;
use crate::layering::layer::{LayerId, LayerList};
use crate::layouter::{LayoutContext, LayoutElementId, LayoutElementNode};

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

            match &node.node_type {
                NodeType::Element(ref _el_data) => {
                    // if let Some(ref style) = el_data.computed_style {
                    //     if let Some(ref bg_color) = style.background_color {
                    //         cr.set_source_rgba(bg_color.r, bg_color.g, bg_color.b, bg_color.a);
                    //         cr.rectangle(el.box_model.content_box().x, el.box_model.content_box().y, el.box_model.content_box().width, el.box_model.content_box().height);
                    //         _ = cr.fill();
                    //     }
                    // }
                }
                NodeType::Text(_text, _style) => {
                    if let Some(LayoutContext::Text(ctx)) = &el.context {
                        let x = el.box_model.content_box().x;
                        let y = el.box_model.content_box().y;
                        rasterize_text_layout(cr, ctx.layout.clone(), (x, y));
                    }
                }
            }
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

            draw_paint(cr, layer_list, el);
        }
    }

    for (layer_id, visible) in visible_layer_list.iter().enumerate() {
        if *visible {
            draw_layer(cr, &layer_list, layer_id as LayerId, wireframed, hover);
        }
    }
}

fn rasterize_text_layout(cr: &Context, layout: gtk4::pango::Layout, offset: (f64, f64)) {
    cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
    cr.move_to(offset.0, offset.1);
    show_layout(cr, &layout);
}