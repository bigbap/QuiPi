use crate::platform::opengl::textures::Texture;
use super::super::prelude::Component;

#[derive(Component, Debug, PartialEq)]
pub struct RTexture {
    pub texture: Texture,
    pub texture_dims: glm::Vec2
}
