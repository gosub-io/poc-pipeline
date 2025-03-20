use crate::common::geo::Rect;
use crate::layouter::text::Alignment;
use crate::painter::commands::brush::Brush;

#[derive(Clone, Debug)]
pub struct Text {
    /// The rectangle in which the text should be drawn
    pub rect: Rect,
    /// Font family (can be multiple fonts separated by ,)
    pub font_family: String,
    /// Size of font in pixels
    pub font_size: f64,
    /// Weight of the font 100-700
    pub font_weight: usize,
    /// Height of each line (line-spacing)
    pub line_height: f64,
    /// Actual text
    pub text: String,
    /// Brush to paint the text with
    pub brush: Brush,
    /// Text alignment
    pub alignment: Alignment
}

impl Text {
    pub fn new(rect: Rect, text: &str, font_family: &str, font_size: f64, font_weight: usize, line_height: f64, brush: Brush, alignment: Alignment) -> Self {
        Text {
            rect,
            font_family: font_family.to_string(),
            font_size,
            font_weight,
            line_height,
            text: text.to_string(),
            brush,
            alignment,
        }
    }
}

