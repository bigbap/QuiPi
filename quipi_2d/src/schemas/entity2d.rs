use quipi_core::{
    opengl::{buffer::BufferUsage, draw::DrawMode},
    schemas::shader::DEFAULT_SHADER,
    systems::assets::ObjectConfig
};
use serde::{Serialize, Deserialize};

use crate::{
    components::{
        CBoundingBox2D,
        CCircle,
        CSprite,
        CMesh2D,
        CModelMatrix2D,
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
    pub shape:      Shape2D,

    pub velocity:   Option<CVelocity2D>,
    pub color:      Option<CRGBA>,
    pub texture:    Option<CName>,
    pub b_box:      Option<CBoundingBox2D>,
    
    pub shader:     CName,
    pub camera:     CName,
    pub draw_mode:  DrawMode,
    pub usage:      BufferUsage
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
        let texture = match &self.texture {
            Some(name) => registry.resources.query::<CName>(name.clone()),
            None => vec![]
        };

        // 3. get camera by name
        let binding = registry.entities.query::<CName>(self.camera.clone());
        let Some(camera) = binding.first() else {
            return Err(SchemaEntityError::CameraNotFound.into())
        };

        // 4. build the entity
        let entity = registry.entities.create()?;
        registry.entities.add(&entity, self.tag.clone());
        registry.entities.add(&entity, CMesh2D::new(
            self.to_obj_config(),
            self.draw_mode,
            self.usage
        )?);
        if let Some(b_box) = self.b_box {
            registry.entities.add(&entity, b_box);
        }
        if let Some(velocity) = self.velocity {
            registry.entities.add(&entity, velocity);
        }
        if let Some(color) = self.color {
            registry.entities.add(&entity, color);
        }
        match &self.shape {
            Shape2D::Rect(rect) => registry.entities.add(&entity, rect.clone()),
            Shape2D::Circle(_) => ()
        }
        registry.entities.add(&entity, self.transform);
        registry.entities.add(&entity, CSprite {
            shader: *shader,
            texture: texture.first().cloned(),
            camera: *camera,
        });
        registry.entities.add(&entity, CModelMatrix2D(self.transform.to_matrix()));

        Ok(entity)
    }

    fn from_entity(entity: VersionedIndex, registry: &Registry) -> Option<Self> {
        let Some(sprite) = registry.entities.get::<CSprite>(&entity) else { return None; };

        if let (Some(tag), Some(transform), Some(mesh), Some(shader)) = (
            registry.entities.get::<CTag>(&entity),
            registry.entities.get::<CTransform2D>(&entity),
            registry.entities.get::<CMesh2D>(&entity),
            registry.resources.get::<CName>(&sprite.shader),
        ) {
            let mut schema = Self {
                tag: tag.clone(),
                transform: transform.clone(),
                usage: mesh.usage,
                shader: shader.clone(),
                velocity: registry.entities.get::<CVelocity2D>(&entity).cloned(),
                color: registry.entities.get::<CRGBA>(&entity).cloned(),
                b_box: registry.entities.get::<CBoundingBox2D>(&entity).cloned(),
                texture: match sprite.texture {
                    Some(tex) => registry.resources.get::<CName>(&tex).cloned(),
                    None => None
                },
                ..Self::default()
            };

            if let Some(rect) = registry.entities.get::<CRect>(&entity) {
                schema.shape = Shape2D::Rect(rect.clone());
            } else if let Some(circle) = registry.entities.get::<CCircle>(&entity) {
                schema.shape = Shape2D::Circle(circle.clone());
            }

            return Some(schema)
        }

        None
    }
}

impl SchemaEntity2D {
    pub fn to_obj_config(&self) -> ObjectConfig {
        match &self.shape {
            Shape2D::Rect(rect) => rect.to_config(self.color),
            Shape2D::Circle(circle) => circle.to_config(self.color),
        }
    }
}

impl Default for SchemaEntity2D {
    fn default() -> Self {
        Self {
            tag: CTag { tag: DEFAULT_RECT_TAG.to_string() },
            transform: CTransform2D::default(),
            shape: Shape2D::Rect(CRect {
                center_x: 0.0,
                center_y: 0.0,
                width: 200.0,
                height: 200.0,
            }),
            velocity: None,
            color: Some(CRGBA { value: [0.1, 0.1, 0.1, 1.0] }),
            b_box: None,
            texture: None,
            shader: CName { name: DEFAULT_SHADER.to_string() },
            camera: CName { name: DEFAULT_CAMERA.to_string() },
            draw_mode: DrawMode::Triangles,
            usage: BufferUsage::StaticDraw
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Shape2D {
    Rect(CRect),
    Circle(CCircle)
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
