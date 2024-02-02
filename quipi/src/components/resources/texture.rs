use crate::{
    Component,
    wrappers::opengl::textures::Texture
};

#[derive(Component)]
pub struct RTexture(pub Texture);
