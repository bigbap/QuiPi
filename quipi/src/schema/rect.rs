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
        CRGBA, CChildren,
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
    pub tag: String,
    pub instances: Vec<SchemaRectInstance>,

    pub width: f32,
    pub height: f32,
    pub center_x: f32,
    pub center_y: f32,
    pub shader: String,

    pub usage: BufferUsage
}

impl ISchema for SchemaRect {
    fn build(
        &self,
        registry: &mut Registry,
    ) -> Result<VersionedIndex, SchemaError> {
        let mut children = CChildren {
            list: Vec::<VersionedIndex>::with_capacity(self.instances.len())
        };

        for instance in self.instances.iter() {
            children.list.push(
                self.build_instance(
                    registry,
                    instance
                )?
            );
        }

        Ok(registry.create_entity(&format!("parent_{}", DEFAULT_RECT_TAG))?
            .with(children)?
            .done()?
        )
    }
}

impl IPrefab<SchemaRectInstance> for SchemaRect {
    fn build_instance(
        &self,
        registry: &mut Registry,
        instance: &SchemaRectInstance
    ) -> Result<VersionedIndex, SchemaError> {
        let Some(shader) = registry.get_resource_by_tag(&self.shader) else {
            return Err(SchemaError::ShaderNotFound)
        };
        
        let model = instance.transform.to_matrix();

        Ok(
            registry.create_entity(&self.tag)?
                .with(CMesh::new(self.to_obj_config(instance), self.usage)?)?
                .with(CBoundingBox {
                    right: self.width,
                    bottom: self.height,
                    ..CBoundingBox::default()
                })?
                .with(instance.velocity)?
                .with(instance.transform)?
                .with(CShader { shader })?
                .with(CModelMatrix(model))?
                .done()?
        )
    }
}

impl SchemaRect {
    pub fn to_obj_config(&self, instance: &SchemaRectInstance) -> ObjectConfig {
        let points: Vec<f32> = vec![
            self.center_x - (self.width / 2.0), self.center_y + (self.height / 2.0), 0.0, // top left
            self.center_x + (self.width / 2.0), self.center_y + (self.height / 2.0), 0.0, // top right
            self.center_x + (self.width / 2.0), self.center_y - (self.height / 2.0), 0.0, // bottom right
            self.center_x - (self.width / 2.0), self.center_y - (self.height / 2.0), 0.0 // bottom left
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
            tag: DEFAULT_RECT_TAG.to_string(),
            instances: vec![SchemaRectInstance::default()],
            center_x: 0.0,
            center_y: 0.0,
            width: 200.0,
            height: 200.0,
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
