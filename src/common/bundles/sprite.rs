use crate::common::assets::TextureAsset;
use crate::common::resources::AssetId;
use crate::prelude::qp_common::components::*;
use crate::prelude::qp_ecs::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct CSprite {}

pub fn sprite_bundle(
    quad: CQuad,
    texture_id: AssetId,
    location: (u32, u32),
    color: CColor,
) -> impl Bundle {
    debug_assert!(texture_id.validate::<TextureAsset>());

    (
        CSprite::default(),
        quad,
        CTransform2D::default(),
        CTextureAtlas {
            texture: texture_id,
            location,
        },
        color,
        CModelMatrix::default(),
    )
}
