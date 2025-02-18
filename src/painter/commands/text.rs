use gtk4::pango::FontDescription;
use crate::common::geo::Rect;
use crate::painter::commands::brush::Brush;

#[derive(Clone, Debug)]
pub struct Text {
    pub rect: Rect,
    pub font_description: FontDescription,
    pub font_size: f64,
    pub text: String,
    pub brush: Brush,
}

impl Text {
    pub fn new(rect: Rect, text: &str, font_description: FontDescription, font_size: f64, brush: Brush) -> Self {
        Text {
            rect,
            font_description,
            font_size,
            text: text.to_string(),
            brush,
        }
    }
}

