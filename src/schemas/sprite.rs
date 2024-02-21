use serde::{Serialize, Deserialize};

use crate::modules::ecs::{
    components::{
        sprite::TextureAtlas,
        CQuad,
        CSprite,
        CTag,
        CTransform2D,
        CVelocity2D
    },
    resources::RTexture,
    VersionedIndex
};
use crate::Registry;

use super::ISchema;

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
        registry: &mut Registry,
    ) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
        let texture_atlas = match &self.texture {
            Some(id_as_str) => {
                let Some(id) = registry.get_resource_id(&id_as_str) else {
                    return Err("[sprite schema] texture doesn't exist".into())
                };

                let texture = registry.get_resource::<RTexture>(id).unwrap();

                Some(TextureAtlas {
                    texture: id,
                    texture_dims: texture.texture_dims,
                    active_texture: glm::vec2(0.0, 0.0)
                })
            },
            None => None
        };

        let entity = registry.entities.create();
        registry.entities.add(&entity, CTag { tag: self.tag.clone() });
        if let Some(velocity) = self.velocity {
            registry.entities.add(&entity, velocity);
        }
        registry.entities.add(&entity, self.quad.clone());
        registry.entities.add(&entity, self.transform);
        registry.entities.add(&entity, CSprite::new(
            &self.quad,
            Some(self.color),
            texture_atlas
        ));

        Ok(entity)
    }

    fn from_entity(entity: VersionedIndex, registry: &Registry) -> Option<Self> {
        let Some(sprite) = registry.entities.get::<CSprite>(&entity) else { return None; };

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
                    Some(atlas) => registry.string_interner.get_string(atlas.texture),
                    None => None
                },
                color: sprite.color,
                velocity: registry.entities.get::<CVelocity2D>(&entity).cloned(),
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

#[derive(Debug, thiserror::Error)]
pub enum SchemaEntityError {
    // SchemaRect errors
    #[error("[sprite schema] shader not found")]
    ShaderNotFound,

    #[error("[sprite schema] camera not found")]
    CameraNotFound,

    #[error("Other error")]
    OtherError(
        #[from]
        Box<dyn std::error::Error>
    ),
}
