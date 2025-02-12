use gtk4::cairo::Context;
use crate::document::node::NodeType;
use crate::layering::layer::LayerList;
use crate::layouter::{LayoutContext, LayoutElementNode};
use crate::paint::rasterize::text::rasterize_text_layout;

pub fn rasterize_element(cr: &Context, layer_list: &LayerList, el: &LayoutElementNode) {
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
            if let LayoutContext::Text(ctx) = &el.context {
                let x = el.box_model.content_box().x;
                let y = el.box_model.content_box().y;
                rasterize_text_layout(cr, ctx.layout.clone(), (x, y));
            }
        }
    }
}
