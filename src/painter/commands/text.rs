use crate::common::geo::Rect;
use crate::painter::commands::brush::Brush;

#[derive(Clone, Debug)]
pub struct Text {
    pub rect: Rect,
    pub font_family: String,
    pub font_size: f64,
    pub font_weight: usize,
    pub text: String,
    pub brush: Brush,
}

impl Text {
    pub fn new(rect: Rect, text: &str, font_family: &str, font_size: f64, font_weight: usize, brush: Brush) -> Self {
        Text {
            rect,
            font_family: font_family.to_string(),
            font_size,
            font_weight,
            text: text.to_string(),
            brush,
        }
    }
}

