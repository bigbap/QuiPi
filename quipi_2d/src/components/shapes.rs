use serde::{Serialize, Deserialize};

use quipi_core::{
    components::CMeshData, systems::assets::ObjectConfig, Component
};

use super::{CBoundingBox2D, CRGBA};

#[derive(Debug, Component, Serialize, Deserialize, Clone, PartialEq)]
pub struct CRect {
    pub width: f32,
    pub height: f32,
    pub center_x: f32,
    pub center_y: f32
}

impl CRect {
    pub fn to_mesh(&self, color: Option<CRGBA>) -> CMeshData {
        let vertices: Vec<f32> = vec![
            self.center_x + (self.width / 2.0), self.center_y + (self.height / 2.0), 0.0, // top right
            self.center_x + (self.width / 2.0), self.center_y - (self.height / 2.0), 0.0, // bottom right
            self.center_x - (self.width / 2.0), self.center_y - (self.height / 2.0), 0.0, // bottom left
            self.center_x - (self.width / 2.0), self.center_y + (self.height / 2.0), 0.0, // top left
        ];
        let colors: Vec<f32> = match color {
            Some(color) => vec![
                color.value[0], color.value[1], color.value[2], color.value[3],
                color.value[0], color.value[1], color.value[2], color.value[3],
                color.value[0], color.value[1], color.value[2], color.value[3],
                color.value[0], color.value[1], color.value[2], color.value[3],
            ],
            _ => vec![]
        };
        let tex_coords: Vec<f32> = vec![
            1.0, 1.0,
            1.0, 0.0,
            0.0, 0.0,
            0.0, 1.0
        ];
        let indices = vec![
            0, 1, 3,
            1, 2, 3
        ];

        CMeshData {
            indices,
            vertices,
            colors,
            tex_coords,
            normals: vec![]
        }
    }

    pub fn to_b_box(&self) -> CBoundingBox2D {
        CBoundingBox2D {
            right: self.width,
            bottom: self.height,
            ..CBoundingBox2D::default()
        }
    }
}

impl Default for CRect {
    fn default() -> Self {
        Self {
            center_x: 0.0,
            center_y: 0.0,
            width: 200.0,
            height: 200.0
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