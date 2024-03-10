use crate::common::resources::AssetId;
use crate::prelude::qp_ecs::*;

#[derive(Debug, Component, Clone, PartialEq)]
pub struct CTexture {
    pub texture: AssetId,
}

#[derive(Debug, Component, Clone, PartialEq)]
pub struct CTextureAtlas {
    pub texture: AssetId,
    pub location: (u32, u32),
}
