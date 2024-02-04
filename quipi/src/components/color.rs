use serde::{Serialize, Deserialize};

use crate::Component;

/**
* RGBA color
* (f32, f32, f32, f32)
*/
#[derive(Debug, Component, Default, Serialize, Deserialize, Clone, Copy)]
pub struct CRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl CRGBA {
    pub fn to_tuple(&self) -> (f32, f32, f32, f32) {
        (
            self.r,
            self.g,
            self.b,
            self.a
        )
    }
}
