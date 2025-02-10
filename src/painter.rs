mod texture_store;

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
// Note that we do not specify what the data contains. It could be an specific image format from the
// used painting backend.
#[derive(Debug)]
pub struct Texture {
    id: TextureId,
    width: usize,
    height: usize,
    data: Vec<u8>,
}

pub struct Painter {}

impl Painter {
    pub(crate) fn paint(&self, tile: &mut Tile) -> Arc<Texture> {
        // Paint the tile
        // tile.elements.iter().for_each(|element| {
        //     // Paint the element
        // });

        let binding = get_texture_store();
        let store = binding.write().unwrap();

        let mut surface = cairo::ImageSurface::create(cairo::Format::ARgb32, tile.rect.width as i32, tile.rect.height as i32).expect("Failed to create image surface");
        let cr = cairo::Context::new(&surface).unwrap();

        let mut rng = rand::rng();
        cr.set_source_rgb(
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
        );

        cr.move_to(100.0, 100.0);
        cr.show_text(format!("Tile {}", tile.id).as_str());
        let _ = cr.stroke();

        surface.flush();

        dbg!(&surface.data());
        dbg!(&surface.height());
        dbg!(&surface.width());

        // let img = image::open("sub.png").expect("Failed to open image");
        let x = Arc::new(Texture {
            id: store.next_id(),
            width: surface.width() as usize,
            height: surface.height() as usize,
            data: vec![],
        });

        println!("Painted tile: {}", tile.id);

        x
    }
}

impl Painter {
    pub(crate) fn new() -> Painter {
        Self {}
    }
}
