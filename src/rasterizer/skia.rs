use skia_safe::{Bitmap, Canvas, Matrix, Paint, Rect, SamplingOptions, TileMode};
use crate::painter::commands::PaintCommand;
use crate::rasterizer::Rasterable;
use crate::common::texture::TextureId;
use crate::common::get_texture_store;
use crate::tiler::Tile;

mod rectangle;
mod paint;
mod text;
mod svg;

pub struct SkiaRasterizer;

impl SkiaRasterizer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Rasterable for SkiaRasterizer {
    fn rasterize(&self, tile: &Tile) -> TextureId {
        let width = tile.rect.width as u32;
        let height = tile.rect.height as u32;

        let mut surface = skia_safe::surfaces::raster_n32_premul(
            skia_safe::ISize::new(width as i32, height as i32),
        ).unwrap();

        let canvas = surface.canvas();
        clear_canvas(canvas, (width as i32, height as i32));
        canvas.clip_rect(
            skia_safe::Rect::new(0.0, 0.0, width as f32, height as f32),
            None,
            None,
        );
        canvas.translate((-tile.rect.x as f32, -tile.rect.y as f32));
        // canvas.clear(skia_safe::Color4f::new(0x17 as f32 /255.0, 0x23 as f32 /255.0, 0xa5 as f32 /255.0, 1.0));

        for element in &tile.elements {
            for command in &element.paint_commands {
                match command {
                    PaintCommand::Rectangle(command) => {
                        rectangle::do_paint_rectangle(canvas, &tile, &command);
                    }
                    PaintCommand::Text(command) => {
                        match text::do_paint_text(canvas, &tile, &command) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Failed to paint text: {:?}", e);
                            }
                        }
                    }
                    PaintCommand::Svg(command) => {
                        svg::do_paint_svg(canvas, &tile, command.media_id, &command.rect);
                    }
                }
            }
        }

        let peek = canvas.peek_pixels().unwrap();
        let pixels = peek.bytes().unwrap().to_vec();

        let binding = get_texture_store();
        let mut texture_store = binding.write().expect("Failed to get texture store");
        let texture_id = texture_store.add(width as usize, height as usize, pixels);

        // _ = texture_store.save_to_disk(texture_id);

        texture_id
    }
}


// This creates a checkerboard pattern for the canvas background. It's a simple way to display the
// actual page from the background that may or may not have rendered (ie: margins on body)
fn clear_canvas(canvas: &Canvas, size: (i32, i32)) {
    let tile_size = 8.0;

    let mut bitmap = Bitmap::new();
    bitmap.alloc_n32_pixels((2 * tile_size as i32, 2 * tile_size as i32), true);
    {
        let tmp_canvas = Canvas::from_bitmap(&bitmap, None).unwrap();
        tmp_canvas.clear(skia_safe::Color4f::new(1.0, 1.0, 1.0, 1.0));

        let paint = Paint::new(skia_safe::Color4f::new(0.7, 0.7, 0.7, 1.0), None);
        tmp_canvas.draw_rect(Rect::new(tile_size, 0.0, tile_size * 2.0, tile_size), &paint);
        tmp_canvas.draw_rect(Rect::new(0.0, tile_size, tile_size, tile_size * 2.0), &paint);
    }

    let shader = bitmap
        .as_image()
        .to_shader((TileMode::Repeat, TileMode::Repeat), SamplingOptions::default(), Matrix::i())
        .unwrap();

    let mut paint = Paint::default();
    paint.set_shader(shader);
    canvas.draw_rect(Rect::new(0.0, 0.0, size.1 as f32, size.1 as f32), &paint);
}