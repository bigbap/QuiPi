use crate::{
    Component,
    gfx::texture::ITexture
};

#[derive(Component)]
pub struct RTexture(pub Box<dyn ITexture>);
