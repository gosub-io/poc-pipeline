pub mod commands;

use std::ops::AddAssign;
use rand::Rng;
use crate::painter::commands::PaintCommand;
use crate::tiler::Tile;

pub struct Painter {}

impl Painter {
    // Generate paint commands for the given tile
    pub(crate) fn paint(tile: &Tile) -> Vec<PaintCommand> {
        Vec::new()
    }
}
