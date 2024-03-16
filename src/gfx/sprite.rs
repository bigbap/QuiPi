use crate::prelude::qp_common::components::*;
use crate::prelude::qp_ecs::*;
use serde::{Deserialize, Serialize};

use super::prelude::CRenderLayer;

#[derive(Default)]
pub struct SpriteMetadata {
    pub quad: CQuad,
    pub transform: CTransform2D,
    pub texture: Option<CTexture>,
    pub color: Option<CColor>,
    pub render_layer: CRenderLayer,
}

#[derive(Debug, Component, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct CSprite {}

pub fn sprite_bundle(metadata: SpriteMetadata) -> impl Bundle {
    let mut sprite_bundle = BundleBuilder::default();
    let model = CModelMatrix(metadata.transform.to_matrix());

    sprite_bundle.add_bundle((
        CSprite::default(),
        metadata.quad,
        metadata.transform,
        model,
        metadata.render_layer,
    ));

    if let Some(color) = metadata.color {
        sprite_bundle.add_bundle(color);
    }

    if let Some(texture) = metadata.texture {
        sprite_bundle.add_bundle(texture);
    }

    sprite_bundle
}
