use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TextureAtlas {
    pub texture: u64,
    pub texture_dims: glm::Vec2,
    pub active_texture: glm::Vec2
}