use std::collections::HashMap;
use std::ops::AddAssign;
use std::sync::{Arc, RwLock};
use crate::common::geo::{Coordinate, Dimension, Rect};
use crate::layering::layer::{LayerId, LayerList};
use crate::layouter::{LayoutElementId, LayoutElementNode};
use crate::painter::commands::PaintCommand;
use crate::common::texture::TextureId;

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
pub struct TiledLayoutElement {
    /// Element to layout
    pub id: LayoutElementId,
    /// Position and dimension of the element inside the tile
    pub rect: Rect,
    /// Coordinate of the element in the tile
    pub position: Coordinate,
}

/*

Here is a box element (id 67) centered within 4 tiles. The tiles are 100x50 each.
The rect size is 100x50.

In tile 1, the rect of element 67 is (0, 0, 50, 25). The position is (50, 25)
In tile 2, the rect of element 67 is (50, 0, 50, 25). The position is (0, 25)
In tile 3, the rect of element 67 is (0, 25, 50, 25). The position is (50, 0).
In tile 4, the rect of element 67 is (50, 25, 50, 25). The position is (0, 0).

The position defines where the element will start in the tile.
The rect defines the position and dimension of the element that needs to be rendered.

In the first tile, the element starts at 50x25. Even though the element is 100x50 in side,
the rect starts at 0,0 to 50,25. Which is the top left quarter of the element.

    0           100         200
    +------------+-----------+
    |            |           |
    |        ####|####       |
    |        ####|####       |
    |        ####|####       |
 50 +------------+-----------+
    |        ####|####       |
    |        ####|####       |
    |        ####|####       |
    |            |           |
100 +------------+-----------+
*/

#[derive(Clone, Debug, PartialEq)]
pub enum TileState {
    /// Tile texture is clean
    Clean,
    /// Tile texture needs a repaint
    Dirty,
    /// Tile texture cannot be rendered by this backend
    Unrenderable,
}

#[derive(Debug, Clone)]
pub struct Tile {
    /// Tile ID
    pub id: TileId,
    /// Elements found in the tile
    pub elements: Vec<TiledLayoutElement>,
    /// Texture that this tile is rendered to
    pub texture_id: Option<TextureId>,
    /// List of paint commands to execute in order to draw the elements in this tile
    pub paint_commands: Vec<PaintCommand>,
    /// State of the tile
    pub state: TileState,
    // Position and dimension of the tile in the layer
    pub rect: Rect,
    /// Layer id on which this tile lives
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

    pub default_tile_dimension: Dimension,
}

impl TileList {
    pub fn get_tiles_for_element(&self, element_id: LayoutElementId) -> Vec<TileId> {
        let mut matching_tiles = vec![];

        for tile in self.arena.values() {
            for element in &tile.elements {
                if element.id == element_id {
                    matching_tiles.push(tile.id);
                }
            }
        }

        matching_tiles
    }

    pub fn invalidate_all(&mut self) {
        for tile in self.arena.values_mut() {
            tile.state = TileState::Dirty;
        }
    }

    pub fn invalidate_tile(&mut self, tile_id: TileId) {
        let tile = self.arena.get_mut(&tile_id).unwrap();
        tile.state = TileState::Dirty;
    }

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

        let Some(tile_ids) = self.layers.get(&layer_id) else {
            return matching_tiles
        };

        for tile_id in tile_ids {
            let tile = self.arena.get(tile_id).unwrap();
            if tile.rect.intersects(viewport) {
                matching_tiles.push(*tile_id);
            }
        }
        matching_tiles
    }
}

impl TileList {
    pub fn new(layer_list: LayerList, dimension: Dimension) -> Self {
        Self {
            layer_list: Arc::new(layer_list),
            layers: HashMap::new(),
            arena: HashMap::new(),
            next_node_id: Arc::new(RwLock::new(TileId::new(0))),
            default_tile_dimension: dimension,
        }
    }

    pub fn generate(&mut self) {
        let rows = (self.layer_list.layout_tree.root_dimension.height / self.default_tile_dimension.height).ceil() as usize;
        let cols = (self.layer_list.layout_tree.root_dimension.width / self.default_tile_dimension.width).ceil() as usize;

        let mut layer_list = self.layer_list.layers.read().unwrap();

        // iterate each layer
        for layer_id in self.layer_list.layer_ids.read().unwrap().iter() {
            // Each layer gets a list of tiles (rows * cols). They are stored in the arena.
            let mut tile_ids = Vec::with_capacity(rows * cols);

            // Generate tiles for this layer
            for y in 0..rows {
                for x in 0..cols {
                    let tile_id = self.next_node_id();
                    let tile = Tile {
                        id: tile_id,
                        elements: Vec::new(),
                        texture_id: None,
                        paint_commands: Vec::new(),
                        state: TileState::Dirty,
                        rect: Rect::new(
                            x as f64 * self.default_tile_dimension.width,
                            y as f64 * self.default_tile_dimension.height,
                            self.default_tile_dimension.width,
                            self.default_tile_dimension.height,
                        ),
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
                    log::warn!("Warning: Element {:?} not found in layout tree!", element_id);
                    continue;
                };

                // Iterate all tiles that intersects this element. Calculate the offset and dimension the element in those tiles
                let matching_tile_ids = self.find_intersecting_tiles(&tile_ids, element);
                // println!("Layer {:?} / Element {:?}: {:?}", layer_id, element_id, matching_tile_ids);
                for tile_id in &matching_tile_ids {
                    let tile = self.arena.get_mut(&tile_id).unwrap();

                    if tile.rect.intersects(element.box_model.margin_box) {
                        let rect = element.box_model.margin_box;

                        let position = Coordinate::new(
                            tile.rect.x.max(rect.x) - rect.x,
                            tile.rect.y.max(rect.y) - rect.y
                        );

                        let dimension = Rect::new(
                            rect.x.max(tile.rect.x) - tile.rect.x,
                            rect.y.max(tile.rect.y) - tile.rect.y,
                            (tile.rect.x + tile.rect.width).min(rect.x + rect.width) - tile.rect.x.max(rect.x),
                            (tile.rect.y + tile.rect.height).min(rect.y + rect.height) - tile.rect.y.max(rect.y),
                        );

                        let tiled_element = TiledLayoutElement {
                            id: element_id,
                            rect: dimension,
                            position,
                        };

                        tile.elements.push(tiled_element);
                    }
                }
            }
        }
    }

    fn find_intersecting_tiles(&self, tile_ids: &Vec<TileId>, element: &LayoutElementNode) -> Vec<TileId> {
        let mut matching_tile_ids = vec![];
        for tile_id in tile_ids.iter() {
            let tile = self.arena.get(tile_id).unwrap();
            if tile.rect.intersects(element.box_model.margin_box) {
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
                let tile = self.arena.get(tile_id).unwrap();
                println!("  Tile: {} : {} elements", tile_id, tile.elements.len());
            }
        }
    }

    pub fn next_node_id(&self) -> TileId {
        let mut nid = self.next_node_id.write().expect("Failed to lock next node ID");
        let id = *nid;
        *nid += 1;
        id
    }
}

