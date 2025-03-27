use crate::common::get_media_store;
use crate::common::media::MediaId;
use crate::painter::commands::rectangle::Rectangle;
use crate::tiler::Tile;
use resvg::usvg::Transform;
use skia_safe::{images, Data, ImageInfo};

// At this point we can render an SVG. This is a two-step process: first, we need to render the svg to a pixmap
// for a certain size. Then, the next step is to render that pixmap into an image which is rendered onto the canvas.

// To speed things up, we can render the SVG to a pixmap once and store it in the media store. However, we should
// be able to use the dimension as a caching tag. So if the rect() dimension changes for that SVG, we should re-render
// the SVG to a pixmap and image and store it in the media store.

pub(crate) fn do_paint_svg(
    canvas: &skia_safe::Canvas,
    _tile: &Tile,
    media_id: MediaId,
    rect: &Rectangle,
) {
    println!("Painting SVG: {:?}", media_id);
    let binding = get_media_store().read().unwrap();
    let media = binding.get_svg(media_id);

    // Check if we need to re-render the SVG. This happens when we need a different dimension
    if media.svg.rendered_dimension != rect.rect().dimension() {
        let pixmap_size = media.svg.tree.size().to_int_size();
        let mut pixmap =
            resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
        resvg::render(&media.svg.tree, Transform::default(), &mut pixmap.as_mut());

        let info = ImageInfo::new(
            skia_safe::ISize::new(rect.rect().width as i32, rect.rect().height as i32),
            skia_safe::ColorType::BGRA8888,
            skia_safe::AlphaType::Premul,
            None,
        );

        let data = unsafe { Data::new_bytes(pixmap.data()) };
        media.svg.rendered_image =
            images::raster_from_data(&info, &data, (pixmap.width() * 4) as usize).unwrap();
        media.svg.rendered_dimension = rect.rect().dimension();
    }

    canvas.draw_image(
        &media.svg.rendered_image.as_ref(),
        (rect.rect().x as f32, rect.rect().y as f32),
        None,
    );
}
