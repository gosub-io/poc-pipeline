use gtk4::pango::FontDescription;
use crate::common::geo::Rect;

#[derive(Clone, Debug)]
pub struct Text {
    rect: Rect,
    font_description: FontDescription,
    font_size: f64,
    text: String,
}

impl Text {
    pub fn new(rect: Rect, text: &str, font_description: FontDescription, font_size: f64) -> Self {
        Text {
            rect,
            font_description,
            font_size,
            text: text.to_string(),
        }
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }

    pub fn font_description(&self) -> FontDescription {
        self.font_description.clone()
    }

    pub fn font_size(&self) -> f64 {
        self.font_size
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

