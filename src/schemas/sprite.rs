use serde::{Deserialize, Serialize};

use crate::prelude::Schema;
use crate::QPResult;
use crate::{
    errors::QPError,
    prelude::{
        qp_assets::RTexture,
        qp_ecs::{
            components::{CQuad, CSprite, CTag, CTransform2D, CVelocity2D},
            Index,
        },
        GlobalRegistry,
    },
};

pub const DEFAULT_RECT_TAG: &str = "default_rect";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaSprite {
    pub tag: String,
    pub transform: CTransform2D,
    pub quad: CQuad,

    pub velocity: Option<CVelocity2D>,
    pub color: glm::Vec4,
    pub texture: Option<String>,
}

impl Schema for SchemaSprite {
    fn build_entity(&self, registry: &mut GlobalRegistry) -> QPResult<Index> {
        let texture_atlas = match &self.texture {
            Some(id_as_str) => {
                let Some(id) = registry.assets.get_asset_id(&id_as_str) else {
                    return Err(QPError::SpriteTextureDoesntExist);
                };

                let texture = registry.assets.get::<RTexture>(id).unwrap();

                Some(TextureAtlas {
                    texture: id,
                    texture_dims: texture.texture_dims,
                    active_texture: glm::vec2(0.0, 0.0),
                })
            }
            None => None,
        };

        let entity = registry.entities.create((
            CTag {
                tag: self.tag.clone(),
            },
            self.quad.clone(),
            self.transform,
            CSprite::new(&self.quad, Some(self.color), texture_atlas),
        ));

        if let Some(velocity) = self.velocity {
            registry.entities.add(&entity, velocity);
        }

        Ok(entity)
    }

    fn from_entity(entity: Index, registry: &GlobalRegistry) -> Option<Self> {
        let Some(sprite) = registry.entities.get::<CSprite>(&entity) else {
            return None;
        };

        if let (Some(tag), Some(transform), Some(quad)) = (
            registry.entities.get::<CTag>(&entity),
            registry.entities.get::<CTransform2D>(&entity),
            registry.entities.get::<CQuad>(&entity),
        ) {
            let schema = Self {
                tag: tag.tag.clone(),
                transform: transform.clone(),
                quad: quad.clone(),
                texture: match &sprite.texture_atlas {
                    Some(atlas) => registry.strings().get_string(atlas.texture),
                    None => None,
                },
                color: sprite.color,
                velocity: registry.entities.get::<CVelocity2D>(&entity).cloned(),
            };

            return Some(schema);
        }

        None
    }
}

impl Default for SchemaSprite {
    fn default() -> Self {
        Self {
            tag: DEFAULT_RECT_TAG.to_string(),
            transform: CTransform2D::default(),
            quad: CQuad {
                width: 200.0,
                height: 200.0,
                ..CQuad::default()
            },
            velocity: None,
            texture: None,
            color: glm::vec4(1.0, 1.0, 1.0, 1.0),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TextureAtlas {
    pub texture: u64,
    pub texture_dims: glm::Vec2,
    pub active_texture: glm::Vec2,
}
