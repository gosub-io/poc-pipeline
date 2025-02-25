use crate::common::geo::Rect;
use crate::painter::commands::border::Border;
use crate::painter::commands::brush::Brush;

// @TODO: Radius is actually 2x f64 as both edges can have a different radius.
pub type Radius = f64;

#[derive(Clone, Debug)]
pub struct Rectangle {
    rect: Rect,
    background: Option<Brush>,
    border: Border,
    radius_top: Radius,
    radius_right: Radius,
    radius_bottom: Radius,
    radius_left: Radius,
}

impl Rectangle {
    pub fn new(rect: Rect) -> Self {
        Rectangle {
            rect,
            background: None,
            border: Border::new(0.0, Default::default(), Brush::Solid(Default::default())),
            radius_top: 0.0,
            radius_right: 0.0,
            radius_bottom: 0.0,
            radius_left: 0.0,
        }
    }

    pub fn with_radius_tlrb(mut self, top: Radius, right: Radius, bottom: Radius, left: Radius) -> Self {
        self.radius_top = top;
        self.radius_right = right;
        self.radius_bottom = bottom;
        self.radius_left = left;
        self
    }

    pub fn with_radius(mut self, radius: Radius) -> Self {
        self.radius_top = radius;
        self.radius_right = radius;
        self.radius_bottom = radius;
        self.radius_left = radius;
        self
    }

    pub fn with_background(mut self, brush: Brush) -> Self {
        self.background = Some(brush);
        self
    }

    pub fn with_border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }

    pub fn background(&self) -> Option<&Brush> {
        self.background.as_ref()
    }

    pub fn border(&self) -> &Border {
        &self.border
    }

    pub fn radius(&self) -> (Radius, Radius, Radius, Radius) {
        (self.radius_top, self.radius_right, self.radius_bottom, self.radius_left)
    }
}
