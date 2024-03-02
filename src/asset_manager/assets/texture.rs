use crate::platform::opengl::textures::Texture;
use crate::prelude::qp_ecs::Component;

#[derive(Component, Debug, PartialEq)]
pub struct RTexture {
    pub texture: Texture,
    pub texture_dims: glm::Vec2,
}

// TODO:
#[derive(Component, Debug, PartialEq)]
pub struct RTextureAtlas {
    pub texture: Texture,
    pub texture_dims: glm::Vec2,
}

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
// pub struct TextureAtlas {
//     pub texture: u64,
//     pub texture_dims: glm::Vec2,
//     pub active_texture: glm::Vec2,
// }
