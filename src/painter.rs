pub mod commands;

use std::ops::AddAssign;
use rand::Rng;
use crate::painter::commands::border::{Border, BorderStyle};
use crate::painter::commands::brush::Brush;
use crate::painter::commands::color::Color;
use crate::painter::commands::rectangle::Rectangle;
use crate::painter::commands::PaintCommand;
use crate::tiler::Tile;

pub struct Painter {}

impl Painter {
    // Generate paint commands for the given tile
    pub(crate) fn paint(tile: &Tile) -> Vec<PaintCommand> {
        let mut commands = Vec::new();

        // @TODO: Since we might paint the element partially on this tile, we want to
        // make sure the painting is correct. If we have a bordered rectangle that spans two tiles,
        // we must make sure that this tile (the left side) does not have a border on the right side.
        // The next tile, should have no border on the left side so stitched together, they form a
        // single element.

        // I hope we can draw "outside" the texture surface.. So if the surface is 100x100, and we
        // want to draw a rectangle at 50x50 with a width of 100, it should draw a rectangle that is
        // 50x50 to 100x100. This way we can draw the border on the next tile as well, but we use a
        // negative offset.  This kind of clipping makes it easier to draw elements

        for element in &tile.elements {
            let c = Color::new(
                rand::rng().random_range(0.0..1.0),
                rand::rng().random_range(0.0..1.0),
                rand::rng().random_range(0.0..1.0),
                1.0,
            );
            let brush = Brush::Solid(c);
            let border = Border::new(1.0, BorderStyle::Dotted, Brush::Solid(Color::BLACK));
            let r = Rectangle::new(element.rect).with_background(brush).with_border(border);
            commands.push(PaintCommand::rectangle(r));
        }

        commands
    }
}
