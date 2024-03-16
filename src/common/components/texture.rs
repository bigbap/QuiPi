use crate::{
    assets::{AssetHandle, AssetId},
    gfx::render::assets::Texture,
    prelude::*,
};

#[derive(Debug, Component, Clone, PartialEq)]
pub struct CTexture {
    pub id: AssetHandle<Texture>,
    pub atlas_location: Option<(u32, u32)>,
}
