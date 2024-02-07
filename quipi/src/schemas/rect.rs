use serde::{Serialize, Deserialize};

use crate::{
    systems::assets::ObjectConfig,
    components::{
        CTransform,
        CMesh,
        CModelMatrix,
        CVelocity,
        CBoundingBox,
        CShader,
        CRGBA,
        CChildren,
        CRect,
        CTag,
        CName,
    },
    VersionedIndex,
    Registry,
    wrappers::opengl::buffer::BufferUsage
};

use super::{
    shader::DEFAULT_SHADER,
    SchemaError,
    IPrefab, ISchema,
};

pub const DEFAULT_RECT_TAG: &str = "default_rect";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaRect {
    pub tag: CTag,
    pub instances: Vec<SchemaRectInstance>,

    pub rect: CRect,
    pub shader: String,
    pub usage: BufferUsage
}

impl ISchema for SchemaRect {
    fn build(
        &self,
        registry: &mut Registry,
    ) -> Result<VersionedIndex, SchemaError> {
        let mut instances = Vec::<VersionedIndex>::with_capacity(self.instances.len());
        for params in self.instances.iter() {
            instances.push(self.build_instance(registry, params)?);
        }

        let name = CName { name: DEFAULT_RECT_TAG.to_string() };
        let children = CChildren { list: instances };

        let entity = registry.entities.create()?;
        registry.entities.add(&entity, name);
        registry.entities.add(&entity, children);

        Ok(entity)
    }
}

impl IPrefab<SchemaRectInstance> for SchemaRect {
    fn build_instance(
        &self,
        registry: &mut Registry,
        instance: &SchemaRectInstance
    ) -> Result<VersionedIndex, SchemaError> {
        let filter = CName { name: self.shader.clone() };
        let binding = registry.resources.query::<CName>(filter);
        let Some(shader) = binding.first() else {
            return Err(SchemaError::ShaderNotFound)
        };
        
        let model = instance.transform.to_matrix();
        let b_box = CBoundingBox {
            right: self.rect.width,
            bottom: self.rect.height,
            ..CBoundingBox::default()
        };

        let entity = registry.entities.create()?;
        registry.entities.add(&entity, self.tag.clone());
        registry.entities.add(&entity, CMesh::new(self.to_obj_config(instance), self.usage)?);
        registry.entities.add(&entity, b_box);
        registry.entities.add(&entity, instance.velocity);
        registry.entities.add(&entity, instance.transform);
        registry.entities.add(&entity, CShader { shader: *shader });
        registry.entities.add(&entity, CModelMatrix(model));

        Ok(entity)
    }
}

impl SchemaRect {
    pub fn to_obj_config(&self, instance: &SchemaRectInstance) -> ObjectConfig {
        let CRect { center_x: x, center_y: y, width: w, height: h } = self.rect;

        let points: Vec<f32> = vec![
            x - (w / 2.0), y + (h / 2.0), 0.0, // top left
            x + (w / 2.0), y + (h / 2.0), 0.0, // top right
            x + (w / 2.0), y - (h / 2.0), 0.0, // bottom right
            x - (w / 2.0), y - (h / 2.0), 0.0 // bottom left
        ];
        let colors: Vec<f32> = vec![
            instance.color.r, instance.color.g, instance.color.b, instance.color.a,
            instance.color.r, instance.color.g, instance.color.b, instance.color.a,
            instance.color.r, instance.color.g, instance.color.b, instance.color.a,
            instance.color.r, instance.color.g, instance.color.b, instance.color.a,
        ];
        let indices = vec![
            0, 1, 2,
            3, 0, 2
        ];

        ObjectConfig {
            points,
            indices,
            colors,
            ..ObjectConfig::default()
        }
    }
}

impl Default for SchemaRect {
    fn default() -> Self {
        Self {
            tag: CTag { tag: DEFAULT_RECT_TAG.to_string() },
            instances: vec![SchemaRectInstance::default()],
            rect: CRect {
                center_x: 0.0,
                center_y: 0.0,
                width: 200.0,
                height: 200.0,
            },
            shader: DEFAULT_SHADER.to_string(),
            usage: BufferUsage::StaticDraw
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaRectInstance {
    pub transform: CTransform,
    pub velocity: CVelocity,
    pub color: CRGBA,
}

impl Default for SchemaRectInstance {
    fn default() -> Self {
        Self {
            transform: CTransform::default(),
            velocity: CVelocity::default(),
            color: CRGBA { r: 0.1, g: 0.1, b: 0.1, a: 1.0 }
        }
    }
}
