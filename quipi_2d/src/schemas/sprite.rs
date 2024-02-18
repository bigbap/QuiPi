use quipi_core::{
    components::CDrawable,
    schemas::shader::DEFAULT_SHADER
};
use serde::{Serialize, Deserialize};

use crate::{
    components::{
        CModelMatrix2D, CQuad, CTag, CTransform2D, CVelocity2D
    },
    Registry,
    VersionedIndex
};

use super::{
    camera2d::DEFAULT_CAMERA,
    ISchema
};

pub const DEFAULT_RECT_TAG: &str = "default_rect";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaSprite {
    pub tag:        String,
    pub transform:  CTransform2D,
    pub quad:       CQuad,

    pub velocity:   Option<CVelocity2D>,
    pub color:      Option<glm::Vec4>,
    pub texture:    Option<String>,
    
    pub shader:     String,
    pub camera:     String,
}

impl ISchema for SchemaSprite {
    fn build_entity(
        &self,
        registry: &mut Registry,
    ) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
        let Some(shader) = registry.get_resource_id(&self.shader) else {
            return Err("[entity2d schema] shader doesn't exist".into())
        };
        let Some(camera) = registry.get_resource_id(&self.camera) else {
            return Err("[entity2d schema] camera doesn't exist".into())
        };
        let texture = match &self.texture {
            Some(id_as_str) => {
                let Some(tex) = registry.get_resource_id(&id_as_str) else {
                    return Err("[entity2d schema] texture doesn't exist".into())
                };

                Some(tex)
            },
            None => None
        };

        let entity = registry.entities.create();
        registry.entities.add(&entity, CTag { tag: self.tag.clone() });
        if let Some(velocity) = self.velocity {
            registry.entities.add(&entity, velocity);
        }
        registry.entities.add(&entity, self.quad.to_b_box());
        registry.entities.add(&entity, self.quad.clone());
        registry.entities.add(&entity, CModelMatrix2D(self.transform.to_matrix()));
        registry.entities.add(&entity, self.transform);
        registry.entities.add(&entity, CDrawable {
            shader,
            camera,
            color: self.color,
            texture
        });

        Ok(entity)
    }

    fn from_entity(entity: VersionedIndex, registry: &Registry) -> Option<Self> {
        let Some(drawable) = registry.entities.get::<CDrawable>(&entity) else { return None; };

        if let (Some(tag), Some(transform), Some(quad)) = (
            registry.entities.get::<CTag>(&entity),
            registry.entities.get::<CTransform2D>(&entity),
            registry.entities.get::<CQuad>(&entity),
        ) {
            let schema = Self {
                tag: tag.tag.clone(),
                transform: transform.clone(),
                quad: quad.clone(),
                shader: registry.string_interner.get_string(drawable.shader)?,
                camera: registry.string_interner.get_string(drawable.camera)?,
                texture: match drawable.texture {
                    Some(id) => registry.string_interner.get_string(id),
                    None => None
                },
                color: drawable.color,
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
                color: glm::vec4(0.1, 0.1, 0.1, 1.0),
                ..CQuad::default()
            },
            velocity: None,
            shader: DEFAULT_SHADER.to_string(),
            camera: DEFAULT_CAMERA.to_string(),
            texture: None,
            color: Some(glm::vec4(0.3, 0.3, 0.3, 1.0))
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SchemaEntityError {
    // SchemaRect errors
    #[error("[SchemaRect] shader not found")]
    ShaderNotFound,

    #[error("[SchemaRect] camera not found")]
    CameraNotFound,

    #[error("Other error")]
    OtherError(
        #[from]
        Box<dyn std::error::Error>
    ),
}
