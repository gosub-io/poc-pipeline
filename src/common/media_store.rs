use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use file_type::FileType;
use resvg::usvg;
use crate::common::hash::{hash_from_string, Sha256Hash};
use crate::common::image::Image;
use crate::common::media::{Media, MediaId, MediaImage, MediaSvg, MediaType};
use crate::common::svg::Svg;

const DEFAULT_SVG_ID: MediaId = MediaId::new(0);
const DEFAULT_IMAGE_ID: MediaId = MediaId::new(1);
const FIRST_FREE_IMAGE_ID: u64 = 100;

const DEFAULT_SVG_DATA: &[u8] = include_bytes!("../../resources/not-found.svg");
const DEFAULT_IMAGE_DATA: &[u8] = include_bytes!("../../resources/default-image.png");

/// Media store is global
pub static MEDIA_STORE: OnceLock<RwLock<MediaStore>> = OnceLock::new();

pub fn get_media_store() -> &'static RwLock<MediaStore> {
    MEDIA_STORE.get_or_init(|| RwLock::new(MediaStore::new()))
}

/// Media store keeps all the loaded media in memory so it can be referenced by its MediaID
pub struct MediaStore {
    /// List of all media
    pub entries: RwLock<HashMap<MediaId, Media>>,
    /// List of all images by hash(src)
    pub cache: RwLock<HashMap<Sha256Hash, MediaId>>,
    /// Next media ID
    next_id: RwLock<MediaId>,
}

impl MediaStore {
    pub fn new() -> MediaStore {
        let store = MediaStore {
            entries: RwLock::new(HashMap::new()),
            cache: RwLock::new(HashMap::new()),
            next_id: RwLock::new(MediaId::new(FIRST_FREE_IMAGE_ID)),
        };

        // Add "default svg" to the store.
        let default_svg_tree = usvg::Tree::from_data(&DEFAULT_SVG_DATA, &usvg::Options::default()).expect("Failed to load default svg");
        let mut entries = store.entries.write().expect("Failed to lock images");
        let media = Media::svg("gosub://default/svg", Svg::new(default_svg_tree));
        entries.insert(DEFAULT_SVG_ID, media);
        drop(entries);

        // Add "default image" to the store.
        let default_image = image::load_from_memory(&DEFAULT_IMAGE_DATA).expect("Failed to load default image").to_rgba8();
        let mut entries = store.entries.write().expect("Failed to lock images");
        let media = Media::image("gosub://default/image", Image::new(default_image));
        entries.insert(DEFAULT_IMAGE_ID, media);
        drop(entries);

        store
    }

    /// Load the given media from src into the media store, and return the media ID. Will also store the media(id) in cache
    /// so the next call with the same src will return the same media ID without reloading.
    pub fn load_media(&self, src: &str) -> anyhow::Result<MediaId> {
        // Check if the media from src is already loaded into the cache. If so, return that
        let h = hash_from_string(src);
        let cache = self.cache.read().expect("Failed to lock cache");
        if let Some(media_id) = cache.get(&h) {
            println!("Loading cached media from path: {}", src);
            return Ok(*media_id);
        }
        drop(cache);

        let result = self.load_media_from_source(src);

        // Store it in cache
        if let Ok(media_id) = result{
            let mut cache = self.cache.write().expect("Failed to lock cache");
            cache.insert(h, media_id);
        }

        result
    }

    fn load_media_from_source(&self, src: &str) -> anyhow::Result<MediaId> {
        println!("Loading non-cached media from path: {}", src);
        let Ok((media_type, raw_data)) = self.fetch_resource(src) else {
            return anyhow::bail!("Failed to fetch resource");
        };

        let media = match media_type {
            MediaType::Svg => {
                let svg_tree = match usvg::Tree::from_data(&raw_data, &usvg::Options::default()) {
                    Ok(tree) => tree,
                    Err(_) => {
                        return Err(anyhow::anyhow!("Failed to parse SVG data"));
                    }
                };

                Media::svg(src, Svg::new(svg_tree))
            }
            MediaType::Image => {
                let img = match image::load_from_memory(&raw_data) {
                    Ok(img) => img,
                    Err(_) => {
                        return Err(anyhow::anyhow!("Failed to parse image data"));
                    }
                };

                Media::image(src, img.to_rgba8())
            }
        };

        let media_id = *self.next_id.read().expect("Failed to lock next media ID");
        *self.next_id.write().expect("Failed to lock next media ID") += 1;

        let mut entries = self.entries.write().expect("Failed to lock entries");
        entries.insert(media_id, media);

        Ok(media_id)
    }

    /// Returns a media image. If the media is not an image or does not exist, it will return the default media image id
    pub fn get_image(&self, media_id: MediaId) -> &MediaImage {
       let media = self.get(media_id, MediaType::Image);
       match media {
            Media::Image(image) => &image,
            _ => panic!("Media is not an image"),
       }
    }

    /// Returns a media svg. If the media is not an svg or does not exist, it will return the default media svg id
    pub fn get_svg(&self, media_id: MediaId) -> &MediaSvg {
        let media = self.get(media_id, MediaType::Svg);
        match media {
            Media::Svg(svg) => &svg,
            _ => panic!("Media is not an image"),
        }
    }

    /// Returns a media resource. If the media does not exist, it will return the default media resource as specified by the media_type
    pub fn get(&self, media_id: MediaId, media_type: MediaType) -> &Media {
        let entries = self.entries.read().expect("Failed to lock images");
        if !entries.contains_key(&media_id) {
            return self.default_media(media_type);
        }

        entries.get(&media_id).unwrap()
    }

    /// Returns the default media resource for the given media type
    fn default_media(&self, media_type: MediaType) -> &Media {
        let entries = self.entries.read().expect("Failed to lock images");

        match media_type {
            MediaType::Svg => entries.get(&DEFAULT_SVG_ID).expect("Failed to get default svg"),
            MediaType::Image => entries.get(&DEFAULT_IMAGE_ID).expect("Failed to get default image"),
        }
    }

    /// Fetch resource from the web (or local file system, depending on the src) and returns the media type and raw
    /// bytes. This is blocking.
    fn fetch_resource(&self, src: &str) -> anyhow::Result<(MediaType, &[u8])> {
        let result = reqwest::blocking::get(src);
        let Ok(response) = result else {
            return anyhow::bail!("Failed to fetch resource");
        };

        if !response.status().is_success() {
            return anyhow::bail!("Incorrect http status code returned");
        }

        let detected_content_type = match response.headers().get("content-type") {
            Some(content_type) => Some(FileType::from_media_type(content_type.to_str())),
            None => None,
        };
        dbg!(&detected_content_type);

        // We should be able to detect the file type from the content with just the first 1024 bytes (even less?)
        let raw_bytes = response.bytes().unwrap_or(bytes::Bytes::new());
        // let detected_file_type = Some(FileType::from_bytes(raw_bytes));
        // dbg!(&detected_file_type);
        //
        // if detected_file_type.is_none() {
        //     return anyhow::bail!("Failed to detect file type");
        // }

        if raw_bytes[0] == b'<' {
            return Ok((MediaType::Svg, &raw_bytes));
        }

        Ok((MediaType::Image, &raw_bytes))
    }
}