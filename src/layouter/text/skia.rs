use crate::layouter::text::Alignment;
use std::fmt::Error;
use crate::common::font::skia::get_skia_paragraph;
use crate::common::geo::Dimension;


pub fn get_text_layout(text: &str, font_family: &str, font_size: f64, _font_weight: usize, line_height: f64, max_width: f64, alignment: Alignment) -> Result<Dimension, Error> {
    let paragraph = get_skia_paragraph(text, font_family, font_size, line_height, max_width, alignment, None);

    Ok(Dimension {
        width: paragraph.max_width() as f64,
        height: paragraph.height() as f64,
    })
}
