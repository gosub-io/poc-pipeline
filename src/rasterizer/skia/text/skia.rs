use std::fmt::Error;
use skia_safe::Canvas;
use crate::painter::commands::text::Text;
use crate::common::font::skia::get_skia_paragraph;
use crate::tiler::Tile;

pub fn do_paint_text(canvas: &mut Canvas, tile: &Tile, cmd: &Text) -> Result<(), Error> {
    let paragraph = get_skia_paragraph(cmd.text.as_str(), cmd.font_family.as_str(), cmd.font_size, cmd.line_height, cmd.rect.width, cmd.alignment);
    paragraph.paint(canvas, (-(tile.rect.x - cmd.rect.x) as f32, -(tile.rect.y-cmd.rect.y) as f32));

    Ok(())
}