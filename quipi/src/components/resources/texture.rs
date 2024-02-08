use crate::{
    Component,
    wrappers::opengl::textures::Texture
};

#[derive(Component, Debug, PartialEq)]
pub struct RTexture(pub Texture);
