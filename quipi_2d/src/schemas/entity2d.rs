use quipi_core::{
    schemas::shader::DEFAULT_SHADER,
    components::{
        CDrawable,
        CTexture
    }
};
use serde::{Serialize, Deserialize};

use crate::{
    components::{
        CName,
        CRect,
        CTag,
        CTransform2D,
        CVelocity2D,
        CRGBA
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
pub struct SchemaEntity2D {
    pub tag:        CTag,
    pub transform:  CTransform2D,
    pub rect:       CRect,

    pub velocity:   Option<CVelocity2D>,
    pub color:      Option<CRGBA>,
    pub texture:    Option<CName>,
    
    pub shader:     CName,
    pub camera:     CName,
    pub is_static:  bool,
}

impl ISchema for SchemaEntity2D {
    fn build(
        &self,
        registry: &mut Registry,
    ) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
        // 1. get shader by name
        let binding = registry.resources.query::<CName>(self.shader.clone());
        let Some(shader) = binding.first() else {
            return Err(SchemaEntityError::ShaderNotFound.into())
        };

        // 2. get texture by name if any
        let textures = match &self.texture {
            Some(name) => registry.resources.query::<CName>(name.clone()),
            None => vec![]
        };

        // 3. get camera by name
        let binding = registry.entities.query::<CName>(self.camera.clone());
        let Some(camera) = binding.first() else {
            return Err(SchemaEntityError::CameraNotFound.into())
        };

        // 4. build the entity
        let entity = registry.entities.create();
        registry.entities.add(&entity, self.tag.clone());
        if let Some(velocity) = self.velocity {
            registry.entities.add(&entity, velocity);
        }
        if let Some(texture) = textures.first() {
            registry.entities.add(&entity, CTexture(*texture))
        }
        registry.entities.add(&entity, self.rect.to_mesh(self.color));
        registry.entities.add(&entity, self.rect.to_b_box());
        registry.entities.add(&entity, self.rect.clone());
        registry.entities.add(&entity, self.transform.to_matrix());
        registry.entities.add(&entity, self.transform);
        registry.entities.add(&entity, CDrawable {
            shader: *shader,
            camera: *camera,
        });

        Ok(entity)
    }

    fn from_entity(entity: VersionedIndex, registry: &Registry) -> Option<Self> {
        let Some(drawable) = registry.entities.get::<CDrawable>(&entity) else { return None; };

        if let (Some(tag), Some(transform), Some(shader), Some(camera), Some(rect)) = (
            registry.entities.get::<CTag>(&entity),
            registry.entities.get::<CTransform2D>(&entity),
            registry.resources.get::<CName>(&drawable.shader),
            registry.entities.get::<CName>(&drawable.camera),
            registry.entities.get::<CRect>(&entity),
        ) {
            let schema = Self {
                tag: tag.clone(),
                transform: transform.clone(),
                rect: rect.clone(),
                shader: shader.clone(),
                camera: camera.clone(),
                velocity: registry.entities.get::<CVelocity2D>(&entity).cloned(),
                color: registry.entities.get::<CRGBA>(&entity).cloned(),
                texture: match registry.entities.get::<CTexture>(&entity) {
                    Some(tex) => registry.resources.get::<CName>(&tex.0).cloned(),
                    None => None
                },
                is_static: true, // TODO: this is currently hardcoded
            };

            return Some(schema)
        }

        None
    }
}

impl Default for SchemaEntity2D {
    fn default() -> Self {
        Self {
            tag: CTag { tag: DEFAULT_RECT_TAG.to_string() },
            transform: CTransform2D::default(),
            rect: CRect {
                center_x: 0.0,
                center_y: 0.0,
                width: 200.0,
                height: 200.0,
            },
            velocity: None,
            color: Some(CRGBA { value: [0.1, 0.1, 0.1, 1.0] }),
            texture: None,
            shader: CName { name: DEFAULT_SHADER.to_string() },
            camera: CName { name: DEFAULT_CAMERA.to_string() },
            is_static: true
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
