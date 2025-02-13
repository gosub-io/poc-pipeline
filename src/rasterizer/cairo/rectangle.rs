use gtk4::cairo::Context;
use gtk4::gdk_pixbuf::{Colorspace, Pixbuf};
use gtk4::glib::Bytes;
use gtk4::prelude::GdkCairoContextExt;
use crate::painter::commands::border::BorderStyle;
use crate::painter::commands::brush::Brush;
use crate::painter::commands::rectangle::Rectangle;

fn set_brush(cr: &Context, brush: &Brush) {
    match brush {
        Brush::Solid(color) => {
            cr.set_source_rgba(color.r() as f64, color.g() as f64, color.b() as f64, color.a() as f64);
        }
        Brush::Image(img) => {
            let bytes = Bytes::from_owned(img.data());
            let pixbuf = Pixbuf::from_bytes(&bytes, Colorspace::Rgb, true, 8, img.width() as i32, img.height() as i32, img.width() as i32 * 4);
            cr.set_source_pixbuf(&pixbuf, 0.0, 0.0);
        }
    }
}

fn do_paint_rectangle(cr: &Context, rectangle: Rectangle) {
    // Create initial rect
    match rectangle.background() {
        Some(brush) => {
            cr.rectangle(rectangle.rect().x, rectangle.rect().y, rectangle.rect().width, rectangle.rect().height);
            set_brush(cr, brush);
            _ = cr.fill();
        }
        None => {}
    }

    // Create border
    cr.rectangle(rectangle.rect().x, rectangle.rect().y, rectangle.rect().width, rectangle.rect().height);
    cr.set_line_width(rectangle.border().width() as f64);
    match rectangle.border().style() {
        BorderStyle::None => {}
        BorderStyle::Solid => {
            _ = cr.stroke();
        }
        BorderStyle::Dashed => {
            cr.set_dash(&[50.0, 10.0, 10.0, 10.0], 0.0);
            _ = cr.stroke();
        }
        BorderStyle::Dotted => {
            cr.set_dash(&[10.0, 10.0], 0.0);
            _ = cr.stroke();
        }
        BorderStyle::Double => {}
        BorderStyle::Groove => {}
        BorderStyle::Ridge => {}
        BorderStyle::Inset => {}
        BorderStyle::Outset => {}
        BorderStyle::Hidden => {}
    }
    cr.rectangle(rectangle.rect().x, rectangle.rect().y, rectangle.rect().width, rectangle.rect().height);
}