use skia_safe::{Color, Paint};
use crate::common::geo::Rect;
use crate::painter::commands::brush::Brush;

pub fn set_paint(brush: &Brush, _rect: Rect) -> Paint {
    match brush {
        Brush::Solid(color) => {
            let mut p = Paint::default();
            p.set_color(Color::from_argb(color.a8(), color.r8(), color.g8(), color.b8()));

            p
        }
        Brush::Image(img) => {
            let mut p = Paint::default();
            p.set_image_filter(skia_safe::image_filters::image(
                img.image(),
                None,
                None,
                None,
            ));
        }
    }
}