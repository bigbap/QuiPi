use crate::{
    Component,
    facades::opengl::textures::ITexture
};

#[derive(Component)]
pub struct RTexture(pub Box<dyn ITexture>);
