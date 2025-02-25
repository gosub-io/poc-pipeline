use std::ops::AddAssign;
use image::ImageFormat;
use sha2::{Digest, Sha256};

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

#[derive(Clone)]
pub struct Image {
    /// Width of the image in pixels
    pub width: usize,
    /// Height of the image in pixels
    pub height: usize,
    /// Binary data that represents the image
    pub data: Vec<u8>,
    /// Actual image format of the binary data
    pub format: ImageFormat,
    /// Sha256 hash of the image (not really used yet)
    pub hash: [u8; 32],
}

impl Image {
    #[allow(unused)]
    pub fn new(width: usize, height: usize, data: Vec<u8>, format: ImageFormat) -> Image {
        let mut hasher = Sha256::new();
        hasher.update(data.as_slice());
        let hash = hasher.finalize();

        Image {
            width,
            height,
            data,
            format,
            hash: hash.into(),
        }
    }
}

impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Image")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("format", &self.format)
            .field("size", &self.data.len())
            .field("hash", &hex::encode(&self.hash))
            .finish()
    }
}
