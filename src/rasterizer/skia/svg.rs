use resvg::usvg::Transform;
use skia_safe::{images, Data, ImageInfo};
use crate::common::get_svg_store;
use crate::common::svg::SvgId;
use crate::painter::commands::rectangle::Rectangle;
use crate::tiler::Tile;

pub(crate) fn do_paint_svg(canvas: &skia_safe::Canvas, _tile: &Tile, svg_id: SvgId, rect: &Rectangle) {
    let store = get_svg_store();
    let svg = store.read().unwrap().get(svg_id).unwrap();

    let pixmap_size = svg.tree.size().to_int_size();
    let mut pixmap = resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(&svg.tree, Transform::default(), &mut pixmap.as_mut());

    // // Ok, so we render from tiny_skia::pixmap, to skia_safe::Bitmap, to skia_safe::Image, to skia_safe::Canvas.
    // // This sounds absolutely like the most efficient way to do this...
    // let mut bitmap = Bitmap::new();
    // bitmap.alloc_n32_pixels(
    //     (pixmap.width() as i32, pixmap.height() as i32),
    //     Some(true)
    // );
    // bitmap.set_pixel_ref(Some(pixmap.data()), (0, 0));


    let info = ImageInfo::new(
        skia_safe::ISize::new(rect.rect().width as i32, rect.rect().height as i32),
        skia_safe::ColorType::RGBA8888,
        skia_safe::AlphaType::Premul,
        None,
    );

    let data = unsafe { Data::new_bytes(pixmap.data()) };
    let image = images::raster_from_data(&info, &data, (pixmap.width() * 4) as usize).unwrap();
    canvas.draw_image(&image.as_ref(), (0.0, 0.0), None);
}