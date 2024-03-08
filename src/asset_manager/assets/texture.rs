use crate::platform::opengl::textures::Texture;
use crate::prelude::qp_ecs::*;

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
