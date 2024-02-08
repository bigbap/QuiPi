use crate::Component;
use serde::{Serialize, Deserialize};


#[derive(Debug, Component, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct CSpeed(f32);

/**
* 3D velocity vector
*/
#[derive(Debug, Component, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct CVelocity {
    pub x: f32,
    pub y: f32,
    pub z: f32
}
