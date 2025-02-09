use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::tiler::{Tile, TileId, TileTexture};

pub struct TileTextureCache {
    cache: RwLock<HashMap<TileId, Arc<TileTexture>>>,
}

impl TileTextureCache {
    pub fn new() -> Self {
        TileTextureCache {
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub fn get(&self, tile_id: TileId) -> Option<Arc<TileTexture>> {
        log::info!("Getting tile texture for tile_id: {}", tile_id);
        self.cache.read().unwrap().get(&tile_id).cloned()
    }

    pub fn insert(&mut self, tile: Tile) {
        log::info!("Inserting tile texture for tile_id: {}", tile.id);
        let tile_id = tile.id;
        if tile.texture.is_some() {
            self.cache.write().unwrap().insert(tile_id, tile.texture.unwrap());
        }
    }

    pub fn remove(&mut self, tile_id: TileId) {
        log::info!("Removing tile texture for tile_id: {}", tile_id);
        self.cache.write().unwrap().remove(&tile_id);
    }
}