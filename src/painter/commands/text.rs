use crate::utils::geo::Rect;

#[derive(Clone, Debug)]
pub struct Text {
    rect: Rect,
}

impl Text {
    pub fn new(rect: Rect) -> Self {
        Text { rect }
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }
}

