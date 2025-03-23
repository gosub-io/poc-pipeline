use skia_safe::{AlphaType, Color4f, ColorType, Data, ISize, ImageInfo, Paint, SamplingOptions};
use crate::painter::commands::brush::Brush;

pub fn create_paint(brush: &Brush) -> Paint {
    match brush {
        Brush::Solid(color) => {
            Paint::new(Color4f::new(color.r(), color.g(), color.b(), color.a()), None)
        }
        Brush::Image(img) => {
            let mut p = Paint::default();

            let img_info = ImageInfo::new(
                ISize::new(img.width() as i32, img.height() as i32),
                ColorType::RGBA8888,
                AlphaType::Premul,
                None,
            );

            let skia_img = skia_safe::images::raster_from_data(
                &img_info,
                // @TODO: We don't need to copy, just use img.data() in unsafe{} block
                Data::new_copy(img.data()),
                (img_info.width() * 4) as usize,
            ).unwrap();
            let shader = skia_safe::shaders::image(
                skia_img,
                (skia_safe::TileMode::Clamp, skia_safe::TileMode::Clamp),
                &SamplingOptions::default(),
                None,
            );

            p.set_shader(shader);

            p
        }
    }
}