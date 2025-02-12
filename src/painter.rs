pub mod texture_store;
mod commands;

use std::ops::AddAssign;
use std::sync::Arc;
use gtk4::cairo;
use rand::Rng;
use crate::painter::texture_store::get_texture_store;
use crate::tiler::{Tile, TileList};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureId(u64);

impl TextureId {
    pub const fn new(val: u64) -> Self {
        Self(val)
    }
}

impl AddAssign<i32> for TextureId {
    fn add_assign(&mut self, rhs: i32) {
        self.0 += rhs as u64;
    }
}

impl std::fmt::Display for TextureId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TextureId({})", self.0)
    }
}

// Texture is a simple structure that holds the texture data. It's raw data, width, height and an id.
// Note that we do not specify what the data contains. It could be a specific image format from the
// used painting backend (like ImageSurface data for cairo)
#[derive(Debug)]
pub struct Texture {
    pub id: TextureId,
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

pub struct Painter {}

impl Painter {
    // Paint the given tile with a new texture
    pub(crate) fn paint(&self, tile: &Tile) -> TextureId {
        let mut surface = cairo::ImageSurface::create(cairo::Format::ARgb32, tile.rect.width as i32, tile.rect.height as i32).expect("Failed to create image surface");

        {
            // Each tile gets a number of paint commands. We need to execute these paint commands.
            let cr = cairo::Context::new(&surface).unwrap();

            let mut rng = rand::rng();
            cr.set_source_rgba(
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                0.75,
            );
            cr.arc(100.0, 100.0, 50.0, 0.0, 2.0 * std::f64::consts::PI);
            cr.fill();

            cr.set_source_rgba(
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                0.75,
            );
            cr.move_to(100.0, 100.0);
            cr.set_font_size(20.0);
            cr.show_text(format!("{}", tile.id).as_str());
            let _ = cr.stroke();

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

impl Painter {
    pub(crate) fn new() -> Painter {
        Self {}
    }
}
