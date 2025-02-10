/// A simple rectangle with a position (x, y) and dimensions (width, height).
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    /// Create a new rectangle with the given position and dimensions.
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }

    /// Check if this rectangle intersects with another rectangle.
    pub fn intersects(&self, other: Rect) -> bool {
        self.x < other.x + other.width &&
            self.x + self.width > other.x &&
            self.y < other.y + other.height &&
            self.y + self.height > other.y
    }
}


#[derive(Clone, Debug)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

impl Coordinate {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}