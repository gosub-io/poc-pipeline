//     if show_tilegrid {
//         // Display the tilegrid based on the tiles
//         let h = tile_list.layer_list.layout_tree.root_height;
//         let w = tile_list.layer_list.layout_tree.root_width;
//
//         let row_cnt = (h / tile_list.tile_height as f32).ceil() as usize;
//         let col_cnt = (w / tile_list.tile_width as f32).ceil() as usize;
//
//         cr.set_source_rgba(0.0, 0.0, 0.0, 0.25);
//         for y in 0..row_cnt {
//             for x in 0..col_cnt {
//                 cr.rectangle(
//                     x as f64 * tile_list.tile_width as f64,
//                     y as f64 * tile_list.tile_height as f64,
//                     tile_list.tile_width as f64,
//                     tile_list.tile_height as f64
//                 );
//                 _ = cr.stroke();
//             }
//         }
//     }

mod rectangle;
mod brush;
mod text;

use gtk4::cairo;
use crate::painter::commands::PaintCommand;
use crate::rasterizer::Rasterable;
use crate::common::texture::TextureId;
use crate::common::get_texture_store;
use crate::rasterizer::cairo::text::pango::do_paint_text;
use crate::tiler::Tile;

pub struct CairoRasterizer {}

impl Rasterable for CairoRasterizer {
    fn rasterize(tile: &Tile) -> TextureId {
        let mut surface = cairo::ImageSurface::create(cairo::Format::ARgb32, tile.rect.width as i32, tile.rect.height as i32).expect("Failed to create image surface");

        {
            // Each tile has a number of paint commands. We need to execute these paint commands in order onto this surface
            let cr = cairo::Context::new(&surface).expect("Failed to create cairo context");

            for command in &tile.paint_commands {
                match command {
                    PaintCommand::Rectangle(command) => {
                        rectangle::do_paint_rectangle(&cr.clone(), &tile, &command);
                    }
                    PaintCommand::Text(command) => {
                        match do_paint_text(&cr.clone(), &tile, &command) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Failed to paint text: {:?}", e);
                            }
                        }
                    }
                }
            }

            surface.flush();
        }

        let w = surface.width() as usize;
        let h = surface.height() as usize;

        let Ok(data) = surface.data() else {
            panic!("Failed to get surface data");
        };

        let binding = get_texture_store();
        let mut texture_store = binding.write().expect("Failed to get texture store");
        let texture_id = texture_store.add(w, h, data.to_vec());

        texture_id
    }
}