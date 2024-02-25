use serde::{Serialize, Deserialize};

use crate::{
    errors::QPError,
    prelude::{
        qp_data::{
            ISchema,
            TextureAtlas
        },
        qp_assets::RTexture,
        qp_ecs::{
            components::{
                CQuad,
                CSprite,
                CTag,
                CTransform2D,
                CVelocity2D
            },
            VersionedIndex
        },
        GlobalRegistry
    }
};
use crate::QPResult;

pub const DEFAULT_RECT_TAG: &str = "default_rect";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaSprite {
    pub tag:                String,
    pub transform:          CTransform2D,
    pub quad:               CQuad,

    pub velocity:           Option<CVelocity2D>,
    pub color:              glm::Vec4,
    pub texture:            Option<String>,
}

impl ISchema for SchemaSprite {
    fn build_entity(
        &self,
        registry: &mut GlobalRegistry,
    ) -> QPResult<VersionedIndex> {
        let texture_atlas = match &self.texture {
            Some(id_as_str) => {
                let Some(id) = registry.asset_manager.get_asset_id(&id_as_str) else {
                    return Err(QPError::SpriteTextureDoesntExist)
                };

                let texture = registry.asset_manager.get::<RTexture>(id).unwrap();

                Some(TextureAtlas {
                    texture: id,
                    texture_dims: texture.texture_dims,
                    active_texture: glm::vec2(0.0, 0.0)
                })
            },
            None => None
        };

        let entity = registry.entity_manager.create();
        registry.entity_manager.add(&entity, CTag { tag: self.tag.clone() });
        if let Some(velocity) = self.velocity {
            registry.entity_manager.add(&entity, velocity);
        }
        registry.entity_manager.add(&entity, self.quad.clone());
        registry.entity_manager.add(&entity, self.transform);
        registry.entity_manager.add(&entity, CSprite::new(
            &self.quad,
            Some(self.color),
            texture_atlas
        ));

        Ok(entity)
    }

    fn from_entity(entity: VersionedIndex, registry: &GlobalRegistry) -> Option<Self> {
        let Some(sprite) = registry.entity_manager.get::<CSprite>(&entity) else { return None; };

        if let (Some(tag), Some(transform), Some(quad)) = (
            registry.entity_manager.get::<CTag>(&entity),
            registry.entity_manager.get::<CTransform2D>(&entity),
            registry.entity_manager.get::<CQuad>(&entity),
        ) {
            let schema = Self {
                tag: tag.tag.clone(),
                transform: transform.clone(),
                quad: quad.clone(),
                texture: match &sprite.texture_atlas {
                    Some(atlas) => registry.strings().get_string(atlas.texture),
                    None => None
                },
                color: sprite.color,
                velocity: registry.entity_manager.get::<CVelocity2D>(&entity).cloned(),
            };

            return Some(schema)
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
            color: glm::vec4(1.0, 1.0, 1.0, 1.0)
        }
    }
}
