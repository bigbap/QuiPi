use crate::Component;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Serialize, Deserialize, Clone, PartialEq)]
pub struct CCircle {
    pub radius: f32,
    pub center_x: f32,
    pub center_y: f32,
    pub theta: f32, // in degrees, must be a factor of 360
}