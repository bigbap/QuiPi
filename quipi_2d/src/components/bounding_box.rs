use serde::{Serialize, Deserialize};
use quipi_core::Component;

#[derive(Debug, Component, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct CBoundingBox2D {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32
}

impl CBoundingBox2D {
    pub fn width(&self) -> f32 {
        (self.right - self.left).abs()
    }
    pub fn height(&self) -> f32 {
        (self.bottom - self.top).abs()
    }
}
