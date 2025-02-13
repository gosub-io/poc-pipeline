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
            cr.arc(tile.rect.width / 2.0, tile.rect.height / 2.0, tile.rect.width / 4.0, 0.0, 2.0 * std::f64::consts::PI);
            _ = cr.fill();

            cr.set_source_rgba(
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                0.75,
            );

            let fs = 14.0;

            let s = format!("{}", tile.id);
            let extends = cr.text_extents(s.as_str()).unwrap();

            let center_x = (tile.rect.width - extends.width()) / 2.0;
            cr.move_to(center_x, tile.rect.height - fs - 2.0);
            cr.set_font_size(fs);
            cr.set_source_rgb(0.0, 0.0, 0.0);
            _ = cr.show_text(s.as_str());

            cr.text_extents(format!("{}", tile.id).as_str());

            cr.rectangle(0.0, 0.0, tile.rect.width, tile.rect.height);
            cr.set_line_width(1.0);
            cr.set_dash(&[5.0, 5.0], 0.0);
            _ = cr.stroke();

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