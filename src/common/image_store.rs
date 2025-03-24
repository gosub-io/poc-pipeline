use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};
use image::RgbaImage;
use crate::common::image::ImageId;

/// Image store is global
pub static IMAGE_STORE: OnceLock<RwLock<ImageStore>> = OnceLock::new();

pub fn get_image_store() -> &'static RwLock<ImageStore> {
    IMAGE_STORE.get_or_init(|| RwLock::new(ImageStore::new()))
}

/// Image store keeps all the loaded images in memory so it can be referenced by its ImageID
pub struct ImageStore {
    /// List of all images
    pub images: RwLock<HashMap<ImageId, Arc<RgbaImage>>>,
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

    pub fn store_from_path(&self, src: &str) -> ImageId {
        let response = reqwest::blocking::get(src).expect("Failed to get image");
        if !response.status().is_success() {
            panic!("Failed to get image");
        }

        let img_data = response.bytes().expect("Failed to get image");
        let rgb_img = match image::load_from_memory(&img_data) {
            Ok(img) => img.to_rgba8(),
            Err(_) => {
                // Load bytes from path 'p':
                let p = "sub.png";
                let img_data = std::fs::read(p).expect("Failed to read image");
                image::load_from_memory(&img_data).expect("Failed to load image").to_rgba8()
            },
        };

        let mut images = self.images.write().expect("Failed to lock images");
        let image_id = *self.next_image_id.read().expect("Failed to lock next image ID");
        images.insert(image_id, Arc::new(rgb_img));
        *self.next_image_id.write().expect("Failed to lock next image ID") += 1;
        image_id
    }

    pub fn get(&self, image_id: ImageId) -> Option<Arc<RgbaImage>> {
        let images = self.images.read().expect("Failed to lock images");
        images.get(&image_id).cloned()
    }
}