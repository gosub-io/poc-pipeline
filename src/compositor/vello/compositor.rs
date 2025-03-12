use vello::kurbo::Affine;
use crate::common::browser_state::get_browser_state;
use crate::common::get_texture_store;
use crate::layering::layer::LayerId;
use vello::peniko::{Blob, Image, ImageFormat};

pub fn vello_compositor(scene: &mut vello::Scene, layer_ids: Vec<LayerId>) {
    for layer_id in layer_ids {
        crate::compositor::vello::compositor::compose_layer(scene, layer_id);
    }
}

pub fn compose_layer(scene: &mut vello::Scene, layer_id: LayerId) {
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

        let surface = Image::new(
            Blob::from(texture.data.clone()),
            ImageFormat::Rgba8,
            texture.width as u32,
            texture.height as u32,
        );

        // let rect = Rect::from_origin_size(
        //     Point::new(tile.rect.x, tile.rect.y),
        //     Size::new(tile.rect.width, tile.rect.height),
        // );
        // scene.fill(
        //     Fill::NonZero,
        //     Affine::IDENTITY,
        //     palette::css::GRAY,
        //     None,
        //     &rect
        // );

        scene.draw_image(
            &surface,
            Affine::translate((tile.rect.x, tile.rect.y)),
        );
    }

}