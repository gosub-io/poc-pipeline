use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};
use image::RgbaImage;
use sha2::{Digest, Sha256};
use crate::common::image::ImageId;

type Sha256Hash = [u8; 32];

const DEFAULT_IMAGE_ID: ImageId = ImageId::new(0);
const DEFAULT_IMAGE_DATA: &[u8] = include_bytes!("../../resources/default-image.png");

/// Image store is global
pub static IMAGE_STORE: OnceLock<RwLock<ImageStore>> = OnceLock::new();

pub fn get_image_store() -> &'static RwLock<ImageStore> {
    IMAGE_STORE.get_or_init(|| RwLock::new(ImageStore::new()))
}

/// Image store keeps all the loaded images in memory so it can be referenced by its ImageID
pub struct ImageStore {
    /// List of all images
    pub images: RwLock<HashMap<ImageId, Arc<RgbaImage>>>,
    /// List of all images by hash(src)
    pub cache: RwLock<HashMap<Sha256Hash, ImageId>>,
    /// Next image ID
    next_image_id: RwLock<ImageId>,
}

impl ImageStore {
    pub fn new() -> ImageStore {
        let store = ImageStore {
            images: RwLock::new(HashMap::new()),
            cache: RwLock::new(HashMap::new()),
            next_image_id: RwLock::new(ImageId::new(1)),
        };

        // Add "default image" to the store. We do not set the cache for this image
        let default_image = image::load_from_memory(&DEFAULT_IMAGE_DATA).expect("Failed to load default image").to_rgba8();
        let mut images = store.images.write().expect("Failed to lock images");
        images.insert(DEFAULT_IMAGE_ID, Arc::new(default_image));
        drop(images);

        store
    }

    pub fn store_from_path(&self, src: &str) -> ImageId {
        let h = create_hash(src);
        let cache = self.cache.read().expect("Failed to lock cache");
        if let Some(image_id) = cache.get(&h) {
            return *image_id;
        }
        drop(cache);

        println!("Loading image from path: {}", src);
        let result = reqwest::blocking::get(src);
        let Ok(response) = result else {
            let mut cache = self.cache.write().expect("Failed to lock cache");
            cache.insert(h, DEFAULT_IMAGE_ID);
            return DEFAULT_IMAGE_ID;
        };

        if !response.status().is_success() {
            // On error, we store the default image for this src
            let mut cache = self.cache.write().expect("Failed to lock cache");
            cache.insert(h, DEFAULT_IMAGE_ID);
            return DEFAULT_IMAGE_ID;
        }

        let img_data = response.bytes().expect("Failed to get image");
        let rgb_img = match image::load_from_memory(&img_data) {
            Ok(img) => img.to_rgba8(),
            Err(_) => {
                // On error, we store the default image for this src
                let mut cache = self.cache.write().expect("Failed to lock cache");
                cache.insert(h, DEFAULT_IMAGE_ID);

                return DEFAULT_IMAGE_ID
            },
        };

        let image_id = *self.next_image_id.read().expect("Failed to lock next image ID");
        *self.next_image_id.write().expect("Failed to lock next image ID") += 1;

        let mut images = self.images.write().expect("Failed to lock images");
        images.insert(image_id, Arc::new(rgb_img));
        let mut cache = self.cache.write().expect("Failed to lock cache");
        cache.insert(h, image_id);

        image_id
    }

    pub fn get(&self, image_id: ImageId) -> Option<Arc<RgbaImage>> {
        let images = self.images.read().expect("Failed to lock images");
        if !images.contains_key(&image_id) {
            return images.get(&DEFAULT_IMAGE_ID).cloned();
        }

        images.get(&image_id).cloned()
    }
}

fn create_hash(data: &str) -> Sha256Hash {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hasher.finalize().into()
}