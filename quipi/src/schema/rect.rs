use serde::{Serialize, Deserialize};

use crate::{
    Component,
    systems::assets::ObjectConfig,
    components::{
        CTransform,
        CMesh,
        CModelMatrix,
        CVelocity,
        CBoundingBox,
        CShader
    },
    VersionedIndex,
    Registry,
    wrappers::opengl::buffer::BufferUsage
};

#[derive(Debug, Component, Serialize, Deserialize, Clone)]
pub struct SchemaRect {
    pub tag: String,

    pub width: f32,
    pub height: f32,
    pub center_x: f32,
    pub center_y: f32,
    pub color: glm::Vec4,

    pub transform: CTransform,
    pub velocity: CVelocity,
    pub shader: String,

    pub usage: BufferUsage
}

impl SchemaRect {
    pub fn build_rect(
        &self,
        registry: &mut Registry,
        shader: &VersionedIndex
    ) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
        registry.create_entity("rect")?
            .with(CMesh::new(self.to_obj_config(), self.usage)?)?
            .with(CBoundingBox {
                right: self.width,
                bottom: self.height,
                ..CBoundingBox::default()
            })?
            .with(self.velocity)?
            .with(self.transform)?
            .with(CShader { shader: *shader })?
            .with(CModelMatrix::default())?
            .done()
    }

    pub fn to_obj_config(&self) -> ObjectConfig {
        let points: Vec<f32> = vec![
            self.center_x - (self.width / 2.0), self.center_y + (self.height / 2.0), 0.0, // top left
            self.center_x + (self.width / 2.0), self.center_y + (self.height / 2.0), 0.0, // top right
            self.center_x + (self.width / 2.0), self.center_y - (self.height / 2.0), 0.0, // bottom right
            self.center_x - (self.width / 2.0), self.center_y - (self.height / 2.0), 0.0 // bottom left
        ];
        let colors: Vec<f32> = vec![
            self.color.x, self.color.y, self.color.z, self.color.w,
            self.color.x, self.color.y, self.color.z, self.color.w,
            self.color.x, self.color.y, self.color.z, self.color.w,
            self.color.x, self.color.y, self.color.z, self.color.w,
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
            tag: "rect".to_string(),
            center_x: 0.0,
            center_y: 0.0,
            width: 200.0,
            height: 200.0,
            color: glm::vec4(0.3, 0.4, 0.1, 1.0),
            transform: CTransform::default(),
            velocity: CVelocity::default(),
            shader: "default".to_string(),
            usage: BufferUsage::StaticDraw
        }
    }
}
