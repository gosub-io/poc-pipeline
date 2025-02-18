use gtk4::cairo::Context;
use gtk4::gdk_pixbuf::{Colorspace, Pixbuf};
use gtk4::glib::Bytes;
use gtk4::prelude::GdkCairoContextExt;
use crate::common::geo::Coordinate;
use crate::painter::commands::brush::Brush;

// Sets the given brush to the context
pub fn set_brush(cr: &Context, brush: &Brush, offset: Coordinate) {
    match brush {
        Brush::Solid(color) => {
            cr.set_source_rgba(color.r() as f64, color.g() as f64, color.b() as f64, color.a() as f64);
        }
        Brush::Image(img) => {
            let bytes = Bytes::from(img.data());
            let pixbuf = Pixbuf::from_bytes(&bytes, Colorspace::Rgb, true, 8, img.width() as i32, img.height() as i32, img.width() as i32 * 4);
            cr.set_source_pixbuf(&pixbuf, offset.x, offset.y);
        }
    }
}