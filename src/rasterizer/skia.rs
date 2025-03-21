use crate::painter::commands::PaintCommand;
use crate::rasterizer::Rasterable;
use crate::common::texture::TextureId;
use crate::common::get_texture_store;
use crate::tiler::Tile;

mod rectangle;
mod paint;
mod text;

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
        canvas.clip_rect(
            skia_safe::Rect::new(0.0, 0.0, width as f32, height as f32),
            None,
            None,
        );

        for element in &tile.elements {
            for command in &element.paint_commands {
                match command {
                    PaintCommand::Rectangle(command) => {
                        rectangle::do_paint_rectangle(&mut canvas, &tile, &command);
                    }
                    PaintCommand::Text(command) => {
                        match text::do_paint_text(&mut canvas, &tile, &command) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Failed to paint text: {:?}", e);
                            }
                        }
                    }
                }
            }
        }

        let peek = canvas.peek_pixels().unwrap();
        let pixels = peek.bytes().unwrap().to_vec();

        let binding = get_texture_store();
        let mut texture_store = binding.write().expect("Failed to get texture store");
        let texture_id = texture_store.add(width as usize, height as usize, pixels);

        texture_id
    }
}