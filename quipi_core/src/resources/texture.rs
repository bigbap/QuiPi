use crate::{
    Component,
    platform::opengl::textures::Texture
};

#[derive(Component, Debug, PartialEq)]
pub struct RTexture {
    pub texture: Texture
}
