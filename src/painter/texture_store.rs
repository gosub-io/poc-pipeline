use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};
use crate::painter::{Texture, TextureId};

pub static TEXTURE_STORE: OnceLock<Arc<RwLock<TextureStore>>> = OnceLock::new();

pub fn get_texture_store() -> Arc<RwLock<TextureStore>> {
    TEXTURE_STORE.get_or_init(|| Arc::new(RwLock::new(TextureStore::new()))).clone()
}

/// Texture store stores all the textures. It can remove textures if needed (memory constraints for instance).
pub struct TextureStore {
    textures: HashMap<TextureId, Arc<Texture>>,
    next_id: Arc<RwLock<TextureId>>,
}

impl TextureStore {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            next_id: Arc::new(RwLock::new(TextureId::new(0))),
        }
    }

    pub fn next_id(&self) -> TextureId {
        let mut nid = self.next_id.write().unwrap();
        let id = *nid;
        *nid += 1;
        id
    }

    pub fn has(&self, texture_id: TextureId) -> bool {
        self.textures.contains_key(&texture_id)
    }

    pub fn get(&self, texture_id: TextureId) -> Option<Arc<Texture>> {
        self.textures.get(&texture_id).cloned()
    }

    pub fn get_mut(&mut self, texture_id: TextureId) -> Option<&mut Arc<Texture>> {
        self.textures.get_mut(&texture_id)
    }

    pub fn remove(&mut self, texture_id: TextureId) -> Option<Arc<Texture>> {
        self.textures.remove(&texture_id)
    }
}
