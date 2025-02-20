use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, OnceLock, RwLock};
use image::ImageFormat;
use crate::common::image::{Image, ImageId};

pub static IMAGE_STORE: OnceLock<RwLock<ImageStore>> = OnceLock::new();

pub fn get_image_store() -> &'static RwLock<ImageStore> {
    IMAGE_STORE.get_or_init(|| RwLock::new(ImageStore::new()))
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

    pub fn store_from_path(&self, _filepath: &str) -> ImageId {
        // @TODO: Overwrite the file with a placeholder image found locally
        let filepath = "sub.png";

        // println!("Store from path: {}", filepath);
        let fmt = ImageFormat::from_path(filepath).expect("Failed to get image format");

        let file = File::open(filepath).expect("Failed to open file");
        let reader = BufReader::new(file);
        let rgb_img = image::load(reader, fmt).expect("Failed to load image").to_rgba8();

        let img = Image::new(rgb_img.width() as usize, rgb_img.height() as usize, rgb_img.into_raw(), fmt);

        let mut images = self.images.write().expect("Failed to lock images");
        let image_id = *self.next_image_id.read().expect("Failed to lock next image ID");
        images.insert(image_id, Arc::new(img));
        *self.next_image_id.write().expect("Failed to lock next image ID") += 1;
        image_id
    }

    pub fn get(&self, image_id: ImageId) -> Option<Arc<Image>> {
        let images = self.images.read().expect("Failed to lock images");
        images.get(&image_id).cloned()
    }
}