use crate::common::geo;

/// Represents the thickness (or spacing) on each side.
#[derive(Debug, Clone, Copy)]
pub struct Edges {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

/// Represents a boxmodel of an element.
#[derive(Clone, Copy)]
pub struct BoxModel {
    /// Rectangle of the context box, the inner box of the element.
    pub content: geo::Rect,
    /// Thickness of the padding on each side.
    pub padding: Edges,
    /// Thickness of the border on each side.
    pub border: Edges,
    /// Thickness of the margin on each side.
    pub margin: Edges,
}

impl BoxModel {
    pub const ZERO: Self = Self {
        content: geo::Rect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 },
        margin: Edges { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
        border: Edges { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
        padding: Edges { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
    };

    pub fn new(
        content: geo::Rect,
        padding: Edges,
        border: Edges,
        margin: Edges,
    ) -> Self {
        Self {
            content,
            padding,
            border,
            margin,
        }
    }

    pub fn content_box(&self) -> geo::Rect {
        self.content
    }

    pub fn padding_box(&self) -> geo::Rect {
        let inner_box = self.content_box();
        geo::Rect {
            x: inner_box.x - self.padding.left,
            y: inner_box.y - self.padding.top,
            width: inner_box.width + (self.padding.left + self.padding.right),
            height: inner_box.height + (self.padding.top + self.padding.bottom),
        }
    }

    pub fn border_box(&self) -> geo::Rect {
        let inner_box = self.padding_box();
        geo::Rect {
            x: inner_box.x - self.border.left,
            y: inner_box.y - self.border.top,
            width: inner_box.width + (self.border.left + self.border.right),
            height: inner_box.height + (self.border.top + self.border.bottom),
        }
    }

    pub fn margin_box(&self) -> geo::Rect {
        let inner_box = self.border_box();
        geo::Rect {
            x: inner_box.x - self.margin.left,
            y: inner_box.y - self.margin.top,
            width: inner_box.width + (self.margin.left + self.margin.right),
            height: inner_box.height + (self.margin.top + self.margin.bottom),
        }
    }
}

impl std::fmt::Debug for BoxModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mb = self.margin_box();
        let cb = self.content_box();
        let pb = self.padding_box();
        let bb = self.border_box();

        f.debug_struct("BoxModel")
            .field("margin_box", &format_args!("[{}, {}, {}, {}]", mb.x, mb.y, mb.width, mb.height))
            .field("border_box", &format_args!("[{}, {}, {}, {}]", bb.x, bb.y, bb.width, bb.height))
            .field("padding_box", &format_args!("[{}, {}, {}, {}]", pb.x, pb.y, pb.width, pb.height))
            .field("content_box", &format_args!("[{}, {}, {}, {}]", cb.x, cb.y, cb.width, cb.height))
            .finish()
    }
}