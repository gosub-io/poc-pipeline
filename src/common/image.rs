use std::ops::AddAssign;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageId(u64);

impl ImageId {
    pub const fn new(val: u64) -> Self {
        Self(val)
    }
}

impl AddAssign<i32> for ImageId {
    fn add_assign(&mut self, rhs: i32) {
        self.0 += rhs as u64;
    }
}

impl std::fmt::Display for ImageId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ImageId({})", self.0)
    }
}

pub type Image = image::RgbaImage;

// impl Image {
//     #[allow(unused)]
//     pub fn new(image: image::RgbaImage) -> Image {
//         Image {
//             image
//         }
//     }
// }

// impl std::fmt::Debug for Image {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Image")
//             .field("width", &self.image.width())
//             .field("height", &self.image.height())
//             .finish()
//     }
// }
