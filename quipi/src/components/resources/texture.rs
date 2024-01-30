use crate::{
    Component,
    wrappers::opengl::textures::ITexture
};

#[derive(Component)]
pub struct RTexture(pub Box<dyn ITexture>);
