use std::fmt::Error;
use crate::painter::commands::text::Text;
use crate::common::font::skia::get_skia_paragraph;
use crate::rasterizer::skia::paint::create_paint;
use crate::tiler::Tile;

pub fn do_paint_text(canvas: &skia_safe::Canvas, _tile: &Tile, cmd: &Text) -> Result<(), Error> {
    let skia_paint = create_paint(&cmd.brush);
    let paragraph = get_skia_paragraph(
        cmd.text.as_str(),
        cmd.font_family.as_str(),
        cmd.font_size,
        cmd.line_height,
        cmd.rect.width,
        cmd.alignment,
        Some(skia_paint.paint()),
    );

    paragraph.paint(canvas, (cmd.rect.x as f32, cmd.rect.y as f32));

    Ok(())
}