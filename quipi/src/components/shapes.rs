use serde::{Serialize, Deserialize};

use crate::{
    Component,
    systems::assets::ObjectConfig
};

use super::CRGBA;

#[derive(Debug, Component, Serialize, Deserialize, Clone, PartialEq)]
pub struct CRect {
    pub width: f32,
    pub height: f32,
    pub center_x: f32,
    pub center_y: f32
}

impl CRect {
    pub fn to_config(&self, color: Option<CRGBA>) -> ObjectConfig {
        let points: Vec<f32> = vec![
            self.center_x - (self.width / 2.0), self.center_y + (self.height / 2.0), 0.0, // top left
            self.center_x + (self.width / 2.0), self.center_y + (self.height / 2.0), 0.0, // top right
            self.center_x + (self.width / 2.0), self.center_y - (self.height / 2.0), 0.0, // bottom right
            self.center_x - (self.width / 2.0), self.center_y - (self.height / 2.0), 0.0 // bottom left
        ];
        let colors: Vec<f32> = match color {
            Some(color) => vec![
                color.r, color.g, color.b, color.a,
                color.r, color.g, color.b, color.a,
                color.r, color.g, color.b, color.a,
                color.r, color.g, color.b, color.a,
            ],
            _ => vec![]
        };
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