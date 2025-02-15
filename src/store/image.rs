use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::ops::AddAssign;
use std::sync::{Arc, OnceLock, RwLock};
use image::ImageFormat;
use sha2::{Digest, Sha256};

pub static IMAGE_STORE: OnceLock<RwLock<ImageStore>> = OnceLock::new();

pub fn get_image_store() -> &'static RwLock<ImageStore> {
    IMAGE_STORE.get_or_init(|| RwLock::new(ImageStore::new()))
}

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
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
    pub format: ImageFormat,
    pub hash: [u8; 32],
}

impl Image {
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


pub struct ImageStore {
    /// List of all images
    pub images: RwLock<HashMap<ImageId, Arc<Image>>>,
    /// Next image ID
    next_image_id: RwLock<ImageId>,
}

impl ImageStore {
    pub fn new() -> ImageStore {
        ImageStore {
            images: RwLock::new(HashMap::new()),
            next_image_id: RwLock::new(ImageId::new(0)),
        }
    }

    pub fn store_from_path(&self, filepath: &str) -> ImageId {
        let fmt = ImageFormat::from_path(filepath).unwrap();

        let file = File::open(filepath).unwrap();
        let reader = BufReader::new(file);
        let rgb_img = image::load(reader, fmt).unwrap().to_rgba8();

        let img = Image::new(rgb_img.width() as usize, rgb_img.height() as usize, rgb_img.into_raw(), fmt);

        let mut images = self.images.write().unwrap();
        let image_id = *self.next_image_id.read().unwrap();
        images.insert(image_id, Arc::new(img));
        *self.next_image_id.write().unwrap() += 1;
        image_id
    }

    pub fn get(&self, image_id: ImageId) -> Option<Arc<Image>> {
        let images = self.images.read().unwrap();
        images.get(&image_id).cloned()
    }
}