use gtk4::cairo::Context;
use pangocairo::functions::show_layout;

pub fn rasterize_text_layout(cr: &Context, layout: gtk4::pango::Layout, offset: (f64, f64)) {
    cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
    cr.move_to(offset.0, offset.1);
    show_layout(cr, &layout);
}