use skia_safe::{AlphaType, ColorType, Data, ISize, ImageInfo};
use crate::common::browser_state::get_browser_state;
use crate::common::get_texture_store;
use crate::layering::layer::LayerId;

pub fn skia_compositor(layer_ids: Vec<LayerId>) -> skia_safe::Surface {
    let mut surface = skia_safe::surfaces::raster_n32_premul(
        ISize::new(1024, 786),
    ).unwrap();
    let canvas = surface.canvas();

    for layer_id in layer_ids {
        compose_layer(&canvas, layer_id);
    }

    surface
}

pub fn compose_layer(canvas: &skia_safe::canvas::Canvas, layer_id: LayerId) {
    let binding = get_browser_state();
    let state = binding.read().expect("Failed to get browser state");

    let tile_ids = state.tile_list.read().expect("Failed to get tile list").get_intersecting_tiles(layer_id, state.viewport);
    for tile_id in tile_ids {
        let binding = state.tile_list.write().expect("Failed to get tile list");
        let Some(tile) = binding.get_tile(tile_id) else {
            log::warn!("Tile not found: {:?}", tile_id);
            continue;
        };

        let Some(texture_id) = tile.texture_id else {
            log::error!("No texture found for tile: {:?}", tile_id);
            continue;
        };

        let binding = get_texture_store();
        let texture_store = binding.read().expect("Failed to get texture store");

        let Some(texture) = texture_store.get(texture_id) else {
            log::error!("No texture found for tile: {:?}", tile_id);
            continue;
        };
        drop(texture_store);

        let image_info = ImageInfo::new(
            ISize::new(texture.width as i32, texture.height as i32),
            ColorType::RGBA8888,
            AlphaType::Premul,
            None,
        );

        let data = unsafe { Data::new_bytes(&texture.data.as_slice()) };

        let img = skia_safe::images::raster_from_data(
            &image_info, &data, texture.width * 4
        ).unwrap();

        canvas.draw_image(
            &img,
            (tile.rect.x.round() as f32, tile.rect.y.round() as f32),
            None,
        );
    }

}