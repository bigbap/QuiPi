use serde::{Serialize, Deserialize};

use crate::{
    components::{
        CBoundingBox,
        CDrawable,
        CMesh,
        CModelMatrix,
        CName,
        CRect,
        CCircle,
        CTag,
        CTransform,
        CVelocity,
        CRGBA
    },
    systems::assets::ObjectConfig,
    wrappers::opengl::buffer::BufferUsage,
    Registry,
    VersionedIndex
};

use super::{
    shader::DEFAULT_SHADER,
    SchemaError,
    ISchema,
};

pub const DEFAULT_RECT_TAG: &str = "default_rect";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaEntity2D {
    pub tag:        CTag,
    pub transform:  CTransform,
    pub shape:      Shape2D,

    pub velocity:   Option<CVelocity>,
    pub color:      Option<CRGBA>,
    pub texture:    Option<String>,
    
    pub shader:     String,
    pub usage:      BufferUsage
}

impl ISchema for SchemaEntity2D {
    fn build(
        &self,
        registry: &mut Registry,
    ) -> Result<VersionedIndex, SchemaError> {
        let filter = CName { name: self.shader.clone() };
        let binding = registry.resources.query::<CName>(filter);
        let Some(shader) = binding.first() else {
            return Err(SchemaError::ShaderNotFound)
        };

        let entity = registry.entities.create()?;
        registry.entities.add(&entity, self.tag.clone());
        registry.entities.add(&entity, CMesh::new(self.to_obj_config(), self.usage)?);
        if let Shape2D::Rect(rect) = &self.shape {
            registry.entities.add(&entity, CBoundingBox {
                right: rect.width,
                bottom: rect.height,
                ..CBoundingBox::default()
            });
        };
        
        if let Some(velocity) = self.velocity {
            registry.entities.add(&entity, velocity);
        }
        registry.entities.add(&entity, self.transform);
        registry.entities.add(&entity, CDrawable {
            shader: *shader,
            texture: None // TODO handle textures
        });
        registry.entities.add(&entity, CModelMatrix(self.transform.to_matrix()));

        Ok(entity)
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
            transform: CTransform::default(),
            shape: Shape2D::Rect(CRect {
                center_x: 0.0,
                center_y: 0.0,
                width: 200.0,
                height: 200.0,
            }),
            velocity: None,
            color: Some(CRGBA { r: 0.1, g: 0.1, b: 0.1, a: 1.0 }),
            texture: None,
            shader: DEFAULT_SHADER.to_string(),
            usage: BufferUsage::StaticDraw
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Shape2D {
    Rect(CRect),
    Circle(CCircle)
}
