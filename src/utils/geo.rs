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

    /// Converts a size and dimension into a rectangle.
    pub fn from_size_dimension(size: Size, dimension: Coordinate) -> Self {
        Self {
            x: dimension.x,
            y: dimension.y,
            width: size.width,
            height: size.height,
        }
    }
}

impl Into<Coordinate> for Rect {
    fn into(self) -> Coordinate {
        Coordinate::new(self.x, self.y)
    }
}

impl Into<Size> for Rect {
    fn into(self) -> Size {
        Size::new(self.width, self.height)
    }
}


/// A coordinate is an X/Y position. Could be negative if needed.
#[allow(unused)]
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

/// Size is a dimension in width and height. Together with a Dimension it forms a Rect.
#[allow(unused)]
#[derive(Clone, Debug)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    #[allow(unused)]
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_intersects() {
        let rect1 = Rect::new(0.0, 0.0, 10.0, 10.0);
        let rect2 = Rect::new(5.0, 5.0, 10.0, 10.0);
        assert!(rect1.intersects(rect2));
    }

    #[test]
    fn test_rect_from_size_dimension() {
        let size = Size::new(10.0, 10.0);
        let dimension = Coordinate::new(5.0, 5.0);
        let rect = Rect::from_size_dimension(size, dimension);
        assert_eq!(rect.x, 5.0);
        assert_eq!(rect.y, 5.0);
        assert_eq!(rect.width, 10.0);
        assert_eq!(rect.height, 10.0);
    }

    #[test]
    fn test_into_coordinate() {
        let rect = Rect::new(10.0, 20.0, 0.0, 0.0);
        let coord: Coordinate = rect.into();
        assert_eq!(coord.x, 10.0);
        assert_eq!(coord.y, 20.0);
    }

    #[test]
    fn test_into_size() {
        let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
        let size: Size = rect.into();
        assert_eq!(size.width, 10.0);
        assert_eq!(size.height, 10.0);
    }

    #[test]
    fn test_coordinate_new() {
        let coord = Coordinate::new(10.0, 20.0);
        assert_eq!(coord.x, 10.0);
        assert_eq!(coord.y, 20.0);
    }

    #[test]
    fn test_size_new() {
        let size = Size::new(10.0, 20.0);
        assert_eq!(size.width, 10.0);
        assert_eq!(size.height, 20.0);
    }
}