use crate::common::assets::TextureAsset;
use crate::prelude::qp_common::components::*;
use crate::prelude::qp_storage::*;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct SpriteMetadata {
    pub quad: CQuad,
    pub transform: CTransform2D,
    pub texture: Option<CTexture>,
    pub color: Option<CColor>,
}

#[derive(Debug, Component, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct CSprite {}

pub fn sprite_bundle(metadata: SpriteMetadata) -> impl Bundle {
    let mut sprite_bundle = BundleBuilder::default();
    let model = CModelMatrix(metadata.transform.to_matrix());

    sprite_bundle.add_bundle((CSprite::default(), metadata.quad, metadata.transform, model));

    if let Some(color) = metadata.color {
        sprite_bundle.add_bundle(color);
    }

    if let Some(texture) = metadata.texture {
        debug_assert!(texture.id.validate::<TextureAsset>());

        sprite_bundle.add_bundle(texture);
    }

    sprite_bundle
}
