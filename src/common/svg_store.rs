use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};
use resvg::usvg;
use sha2::{Digest, Sha256};
use crate::common::svg::{Svg, SvgId};

type Sha256Hash = [u8; 32];

const DEFAULT_SVG_ID: SvgId = SvgId::new(0);
const DEFAULT_SVG_DATA: &[u8] = include_bytes!("../../resources/not-found.svg");

/// Image store is global
pub static SVG_STORE: OnceLock<RwLock<SvgStore>> = OnceLock::new();

pub fn get_svg_store() -> &'static RwLock<SvgStore> {
    SVG_STORE.get_or_init(|| RwLock::new(SvgStore::new()))
}

/// Svg store keeps all the loaded svgs in memory so it can be referenced by its SvgID
pub struct SvgStore {
    /// List of all svgs
    pub entries: RwLock<HashMap<SvgId, Arc<Svg>>>,
    /// List of all images by hash(src)
    pub cache: RwLock<HashMap<Sha256Hash, SvgId>>,
    /// Next svg ID
    next_id: RwLock<SvgId>,
}

impl SvgStore {
    pub fn new() -> SvgStore {
        let store = SvgStore {
            entries: RwLock::new(HashMap::new()),
            cache: RwLock::new(HashMap::new()),
            next_id: RwLock::new(SvgId::new(1)),
        };

        // Add "default svg" to the store. We do not set the cache for this svg
        let default_svg_tree = usvg::Tree::from_data(&DEFAULT_SVG_DATA, &usvg::Options::default()).expect("Failed to load default svg");
        let mut entries = store.entries.write().expect("Failed to lock images");
        entries.insert(DEFAULT_SVG_ID, Arc::new(Svg::new("default", default_svg_tree)));
        drop(entries);

        store
    }

    pub fn store_from_path(&self, src: &str) -> SvgId {
        let h = create_hash(src);
        let cache = self.cache.read().expect("Failed to lock cache");
        if let Some(svg_id) = cache.get(&h) {
            return *svg_id;
        }
        drop(cache);

        println!("Loading svg from path: {}", src);
        let result = reqwest::blocking::get(src);
        let Ok(response) = result else {
            let mut cache = self.cache.write().expect("Failed to lock cache");
            cache.insert(h, DEFAULT_SVG_ID);
            return DEFAULT_SVG_ID;
        };

        if !response.status().is_success() {
            // On error, we store the default svg for this src
            let mut cache = self.cache.write().expect("Failed to lock cache");
            cache.insert(h, DEFAULT_SVG_ID);
            return DEFAULT_SVG_ID;
        }

        let raw_data = response.bytes().expect("Failed to get svg");
        let svg_tree = match usvg::Tree::from_data(&raw_data, &usvg::Options::default()) {
            Ok(tree) => tree,
            Err(_) => {
                let mut cache = self.cache.write().expect("Failed to parse SVG data");
                cache.insert(h, DEFAULT_SVG_ID);
                return DEFAULT_SVG_ID;
            }
        };

        let svg_id = *self.next_id.read().expect("Failed to lock next image ID");
        *self.next_id.write().expect("Failed to lock next image ID") += 1;

        let mut entries = self.entries.write().expect("Failed to lock entries");
        entries.insert(svg_id, Arc::new(Svg::new(src, svg_tree)));
        let mut cache = self.cache.write().expect("Failed to lock cache");
        cache.insert(h, svg_id);

        svg_id
    }

    pub fn get(&self, svg_id: SvgId) -> Option<Arc<Svg>> {
        let images = self.entries.read().expect("Failed to lock images");
        if !images.contains_key(&svg_id) {
            return images.get(&DEFAULT_SVG_ID).cloned();
        }

        images.get(&svg_id).cloned()
    }
}

fn create_hash(data: &str) -> Sha256Hash {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hasher.finalize().into()
}