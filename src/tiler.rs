use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::AddAssign;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use crate::geo::{Coordinate, Rect, Size};
use crate::layering::layer::{LayerId, LayerList};
use crate::layouter::{LayoutElementId, LayoutElementNode};

#[allow(unused)]
mod tile_texture_cache;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileId(u64);

impl TileId {
    pub const fn new(val: u64) -> Self {
        Self(val)
    }
}

impl AddAssign<i32> for TileId {
    fn add_assign(&mut self, rhs: i32) {
        self.0 += rhs as u64;
    }
}

impl std::fmt::Display for TileId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TileId({})", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct TileTexture {
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct TiledLayoutElement {
    /// Element to layout
    pub id: LayoutElementId,
    /// Offset inside the tile
    pub offset: Coordinate,
    /// Dimensions of the element to layout
    pub dimension: Size,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TileState {
    /// Tile texture exists, but needs to be repainted
    Dirty,
    /// Tile texture cannot be rendered by this backend
    Unrenderable,
    /// Tile texture is loaded and is valid
    Rendered,
}

#[derive(Debug, Clone)]
pub struct Tile {
    /// Tile ID
    pub id: TileId,
    /// Elements found in the tile
    pub elements: Vec<TiledLayoutElement>,
    /// Texture that this tile is rendered to
    pub texture: Option<Arc<TileTexture>>,
    /// State of the tile
    pub state: TileState,
    pub offset_x: usize,
    pub offset_y: usize,
    pub width: usize,
    pub height: usize,
    pub layer_id: LayerId,
}

#[derive(Debug, Clone)]
pub struct  TileList {
    /// Wrapped layer list
    pub layer_list: Arc<LayerList>,

    /// A list of tile ids per layer
    pub layers: HashMap<LayerId, Vec<TileId>>,

    /// Arena of layout nodes
    pub arena : HashMap<TileId, Tile>,
    /// Next node ID
    next_node_id: Arc<RwLock<TileId>>,

    pub tile_width: usize,
    pub tile_height: usize,
}

impl TileList {
    pub(crate) fn get_tile_mut(&mut self, tile_id: TileId) -> Option<&mut Tile> {
        self.arena.get_mut(&tile_id)
    }

    /// Returns a reference to the given tile or None when not found
    pub(crate) fn get_tile(&self, tile_id: TileId) -> Option<&Tile> {
        self.arena.get(&tile_id)
    }

    /// Return all the tiles for the specific layer that intersects with the given viewport
    pub(crate) fn get_intersecting_tiles(&self, layer_id: LayerId, viewport: Rect) -> Vec<TileId> {
        let mut matching_tiles = vec![];
        let tile_ids = self.layers.get(&layer_id).unwrap();
        for tile_id in tile_ids {
            let tile = self.arena.get(tile_id).unwrap();
            let tile_rect = Rect::new(
                tile.offset_x as f64,
                tile.offset_y as f64,
                tile.width as f64,
                tile.height as f64,
            );
            if tile_rect.intersects(viewport) {
                matching_tiles.push(*tile_id);
            }
        }
        matching_tiles
    }
}

impl TileList {
    pub fn new(layer_list: LayerList, dimension: usize) -> Self {
        Self {
            layer_list: Arc::new(layer_list),
            layers: HashMap::new(),
            arena: HashMap::new(),
            next_node_id: Arc::new(RwLock::new(TileId::new(0))),
            tile_width: dimension,
            tile_height: dimension,
        }
    }

    pub fn generate(&mut self) {
        // calculate the number of rows / cols for the tiles
        let rows = (self.layer_list.layout_tree.root_height / self.tile_height as f32).ceil() as usize;
        let cols = (self.layer_list.layout_tree.root_width / self.tile_width as f32).ceil() as usize;
        println!("Rows: {}, Cols: {}", rows, cols);

        let mut layer_list = self.layer_list.layers.read().unwrap();

        // iterate each layer
        for layer_id in self.layer_list.layer_ids.read().unwrap().iter() {
            // Each layer gets a list of tiles (rows * cols). They are stored in the arena.
            let mut tile_ids = Vec::with_capacity(rows * cols);

            for y in 0..rows {
                for x in 0..cols {
                    let tile_id = self.next_node_id();
                    let tile = Tile {
                        id: tile_id,
                        elements: Vec::new(),
                        texture: None,
                        state: TileState::Dirty,
                        offset_x: x * self.tile_width,
                        offset_y: y * self.tile_height,
                        width: self.tile_width,
                        height: self.tile_height,
                        layer_id: *layer_id,
                    };

                    self.arena.insert(tile_id, tile);
                    tile_ids.push(tile_id);
                }
            }

            // Store tile_ids in the layer mapping
            self.layers.insert(*layer_id, tile_ids.clone());

            // Get elements in this layer
            let Some(layer) = layer_list.get(&layer_id) else {
                continue;
            };

            // iterate each element in the layer
            for &element_id in &layer.elements {
                // Get element
                let Some(element) = self.layer_list.layout_tree.get_node_by_id(element_id) else {
                    println!("Warning: Element {:?} not found in layout tree!", element_id);
                    continue;
                };

                // Find intersecting tiles for this element
                let matching_tile_ids = self.find_intersecting_tiles(&tile_ids, element);

                println!("Layer {:?} / Element {:?}: {:?}", layer_id, element_id, matching_tile_ids);
            }
        }
    }

    fn find_intersecting_tiles(&self, tile_ids: &Vec<TileId>, element: &LayoutElementNode) -> Vec<TileId> {
        let mut matching_tile_ids = vec![];
        for tile_id in tile_ids.iter() {
            let tile = self.arena.get(tile_id).unwrap();
            let tile_rect = Rect::new(
                tile.offset_x as f64,
                tile.offset_y as f64,
                tile.width as f64,
                tile.height as f64,
            );
            if tile_rect.intersects(element.box_model.margin_box) {
                matching_tile_ids.push(*tile_id);
            }
        }
        matching_tile_ids
    }

    pub fn print_list(&self) {
        println!("Generated tilelist:");
        for (layer_id, tile_ids) in self.layers.iter() {
            println!("Layer: {}", layer_id);
            for tile_id in tile_ids {
                println!("  Tile: {}", tile_id);
            }
        }
    }

    pub fn next_node_id(&self) -> TileId {
        let mut nid = self.next_node_id.write().unwrap();
        let id = *nid;
        *nid += 1;
        id
    }

}

