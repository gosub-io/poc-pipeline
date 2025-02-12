use gtk4::cairo;
use rand::Rng;
use crate::rasterize::texture::TextureId;
use crate::rasterize::texture_store::get_texture_store;
use crate::tiler::Tile;

pub mod texture;
#[allow(unused)]
pub mod texture_store;

pub struct Rasterizer {}

impl Rasterizer {
    // Rasterize the given tile with a new texture
    pub(crate) fn rasterize(tile: &Tile) -> TextureId {
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
            _ = cr.fill();

            cr.set_source_rgba(
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                0.75,
            );
            cr.move_to(100.0, 100.0);
            cr.set_font_size(20.0);
            _ = cr.show_text(format!("{}", tile.id).as_str());
            // _ = cr.stroke();

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