use crate::common::geo;

/// Represents the thickness (or spacing) on each side.
#[derive(Debug, Clone, Copy)]
pub struct Edges {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct BoxModel {
    pub margin_box: geo::Rect,
    pub margin: Edges,
    pub border: Edges,
    pub padding: Edges,
}

impl BoxModel {
    pub const ZERO: Self = Self {
        margin_box: geo::Rect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 },
        margin: Edges { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
        border: Edges { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
        padding: Edges { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
    };

    pub fn border_box(&self) -> geo::Rect {
        geo::Rect {
            x: self.margin_box.x + self.margin.left,
            y: self.margin_box.y + self.margin.top,
            width: self.margin_box.width - (self.margin.left + self.margin.right),
            height: self.margin_box.height - (self.margin.top + self.margin.bottom),
        }
    }

    pub fn padding_box(&self) -> geo::Rect {
        let border_box = self.border_box();
        geo::Rect {
            x: border_box.x + self.border.left,
            y: border_box.y + self.border.top,
            width: border_box.width - (self.border.left + self.border.right),
            height: border_box.height - (self.border.top + self.border.bottom),
        }
    }

    pub fn content_box(&self) -> geo::Rect {
        let padding_box = self.padding_box();
        geo::Rect {
            x: padding_box.x + self.padding.left,
            y: padding_box.y + self.padding.top,
            width: padding_box.width - (self.padding.left + self.padding.right),
            height: padding_box.height - (self.padding.top + self.padding.bottom),
        }
    }
}