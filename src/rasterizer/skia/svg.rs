use resvg::usvg::Transform;
use skia_safe::{images, Data, ImageInfo};
use crate::common::get_media_store;
use crate::common::media::MediaId;
use crate::painter::commands::rectangle::Rectangle;
use crate::tiler::Tile;

pub(crate) fn do_paint_svg(canvas: &skia_safe::Canvas, _tile: &Tile, media_id: MediaId, rect: &Rectangle) {
    let binding = get_media_store().read().unwrap();
    let media = binding.get_svg(media_id);

    let width  = rect.rect().width;
    let height = rect.rect().height;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(width as u32, height as u32).unwrap();

    let transform = Transform::from_scale(
        width as f32 / &media.svg.tree.size().width(),
        height as f32 / &media.svg.tree.size().height(),
    );
    resvg::render(&media.svg.tree, transform, &mut pixmap.as_mut());

    let info = ImageInfo::new(
        skia_safe::ISize::new(width as i32, height as i32),
        skia_safe::ColorType::RGBA8888,
        skia_safe::AlphaType::Premul,
        None,
    );

    let data = unsafe { Data::new_bytes(pixmap.data()) };
    let image = images::raster_from_data(&info, &data, (pixmap.width() * 4) as usize).unwrap();

    canvas.draw_image(&image.as_ref(), (rect.rect().x as f32, rect.rect().y as f32), None);
}