use vello::peniko::{Blob, BrushRef};
use vello::peniko::color::{AlphaColor, Rgba8};
use crate::common::geo::Rect;
use crate::painter::commands::brush::Brush;
use vello::peniko::Image as PenikoImage;

pub fn set_brush(brush: &Brush, _rect: Rect) -> BrushRef {
    match brush {
        Brush::Solid(color) => {
            let c = Rgba8::from_u8_array([color.r8(), color.g8(), color.b8(), color.a8()]);
            BrushRef::Solid(AlphaColor::from(c))
        }
        Brush::Image(img) => {
            BrushRef::Image(&PenikoImage::new(
                Blob::from(img.data().to_vec()),
                vello::peniko::ImageFormat::Rgba8,
                img.width(),
                img.height(),
            ))
        }
    }
}