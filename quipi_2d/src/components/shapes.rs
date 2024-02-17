use serde::{Serialize, Deserialize};

use quipi_core::{
    rendering::{
        batch::IMesh,
        vertex::Vertex
    },
    systems::assets::ObjectConfig,
    Component
};

use super::{CBoundingBox2D, CRGBA};

#[derive(Debug, Component, Serialize, Deserialize, Clone, PartialEq)]
pub struct CQuad {
    pub width: f32,
    pub height: f32,
    pub center_x: f32,
    pub center_y: f32,

    pub mvp: glm::Mat4,
    pub color: glm::Vec4
}

impl CQuad {
    pub fn to_b_box(&self) -> CBoundingBox2D {
        CBoundingBox2D {
            right: self.width,
            bottom: self.height,
            ..CBoundingBox2D::default()
        }
    }
}

impl IMesh for CQuad {
    fn indices() -> Vec<i32> { vec![0, 1, 3, 1, 2, 3]}
    fn vertex_count() -> usize { 4 }

    fn vertices(&self) -> Vec<Vertex> {
        let pos1 = self.mvp * glm::vec4(self.center_x + (self.width / 2.0), self.center_y + (self.height / 2.0), 0.0, 1.0);
        let pos2 = self.mvp * glm::vec4(self.center_x + (self.width / 2.0), self.center_y - (self.height / 2.0), 0.0, 1.0);
        let pos3 = self.mvp * glm::vec4(self.center_x - (self.width / 2.0), self.center_y - (self.height / 2.0), 0.0, 1.0);
        let pos4 = self.mvp * glm::vec4(self.center_x - (self.width / 2.0), self.center_y + (self.height / 2.0), 0.0, 1.0);

        vec![
            Vertex { // top right
                position: glm::vec3(pos1.x, pos1.y, pos1.z),
                color: self.color,
                tex_coords: glm::vec2(1.0, 1.0),
                tex_index: 0.0
            },
            Vertex { // bottom right
                position: glm::vec3(pos2.x, pos2.y, pos2.z),
                color: self.color,
                tex_coords: glm::vec2(1.0, 0.0),
                tex_index: 0.0 // TODO: hardcoded
            },
            Vertex { // bottom left
                position: glm::vec3(pos3.x, pos3.y, pos3.z),
                color: self.color,
                tex_coords: glm::vec2(0.0, 0.0),
                tex_index: 0.0 // TODO: hardcoded
            },
            Vertex { // top left
                position: glm::vec3(pos4.x, pos4.y, pos4.z),
                color: self.color,
                tex_coords: glm::vec2(0.0, 1.0),
                tex_index: 0.0 // TODO: hardcoded
            }
        ]
    }
}

impl Default for CQuad {
    fn default() -> Self {
        Self {
            center_x: 0.0,
            center_y: 0.0,
            width: 200.0,
            height: 200.0,
            mvp: glm::Mat4::identity(),
            color: glm::vec4(1.0, 1.0, 1.0, 1.0)
        }
    }
}

#[derive(Debug, Component, Serialize, Deserialize, Clone, PartialEq)]
pub struct CCircle {
    pub radius: f32,
    pub center_x: f32,
    pub center_y: f32,
    pub theta: f32, // in degrees, must be a factor of 360
}

impl CCircle {
    pub fn to_config(&self, _color: Option<CRGBA>) -> ObjectConfig {
        let _theta = self.theta.clamp(5.0, 90.0);

        // TODO
        let mut _points = vec![self.center_x, self.center_y, 0.0];

        ObjectConfig::default()
    }
}