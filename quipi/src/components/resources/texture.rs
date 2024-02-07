use crate::{
    Component,
    wrappers::opengl::textures::Texture
};

#[derive(Component, PartialEq)]
pub struct RTexture(pub Texture);
