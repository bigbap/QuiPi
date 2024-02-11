use quipi_core::Component;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct CVelocity2D {
    pub x: f32,
    pub y: f32
}