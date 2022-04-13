use std::collections::HashMap;
use log::{error, debug};

use crate::texture::Texture;

pub struct AssetManager {
    textures: HashMap<String, Texture>
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new()
        }
    }
    
    pub fn load_texture(&mut self, path: &str, id: &str) {
        match image::open(path) {
            Ok(i) => {
                match Texture::new(i.width(), i.height(), i.color().channel_count() as u32, Some(i.as_bytes())) {
                    Ok(t) => {
                        self.textures.insert(
                            id.to_string(),
                            t  
                        );
                    },
                    Err(s) => error!("Failed to load texture {}: {}", id, s)
                }
                
                debug!("Loaded texture {}", id);
            },
            Err(e) => error!("Failed to load texture {}: {}", id, e)
        }
    }

    pub fn get_texture(&self, id: &str) -> Option<&Texture> {
        self.textures.get(id)
    }

    pub fn get_mut_texture(&mut self, id: &str) -> Option<&mut Texture> {
        self.textures.get_mut(id)
    }
}