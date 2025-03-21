use crate::tiler::TileId;
use std::fmt::Error;
use vello::kurbo::Affine;
use vello::peniko::Blob;
use vello::Scene;
use crate::painter::commands::text::Text;
use crate::tiler::Tile;
use crate::common::font::skia::get_skia_paragraph;
use crate::tiler;

pub fn do_paint_text(scene: &mut Scene, tile: &Tile, cmd: &Text) -> Result<(), Error> {
    // Get the paragraph from Skia
    let paragraph = get_skia_paragraph(cmd.text.as_str(), cmd.font_family.as_str(), cmd.font_size, cmd.line_height, cmd.rect.width, cmd.alignment);

    // Create a (skia) surface to render onto
    let mut surface = skia_safe::surfaces::raster_n32_premul((tile.rect.width as i32, tile.rect.height as i32)).unwrap();

    // Create the canvas. Let's hope this is actually something in the GPU, and paint the paragraph onto it
    let mut canvas = surface.canvas();
    canvas.clear(skia_safe::Color::TRANSPARENT);

    // paragraph.paint(&mut canvas, (0.0, 0.0));
    // paragraph.paint(&mut canvas, (-tile.rect.x as f32, -tile.rect.y as f32));
    paragraph.paint(&mut canvas, (-(tile.rect.x - cmd.rect.x) as f32, -(tile.rect.y-cmd.rect.y) as f32));

    // let img = surface.image_snapshot();
    // let data = img.encode_to_data(skia_safe::EncodedImageFormat::PNG).unwrap();
    // let b = data.as_bytes();
    // std::fs::write(format!("text-{}.png", tile.id), b).unwrap();

    // // Now, we need to copy the skia surface into a vello scene. It would be very nice if we could find a way
    // // to do this without copying the pixels. If we can find the texture-id of the skia canvas/surface, then we
    // // might be able to use that directly in the vello scene. For now, we use copy stuff (probably).
    let peek = canvas.peek_pixels().unwrap();
    let pixels = peek.bytes().unwrap().to_vec();
    let blob = Blob::from(pixels);
    let img = vello::peniko::Image::new(blob, vello::peniko::ImageFormat::Rgba8, tile.rect.width as u32, tile.rect.height as u32);
    scene.draw_image(&img, Affine::IDENTITY);
    // scene.draw_image(&img, Affine::translate((tile.rect.x, tile.rect.y)));

    Ok(())
}