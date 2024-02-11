use serde::{Serialize, Deserialize};

use crate::Component;

/**
* RGBA color
* (f32, f32, f32, f32)
*/
#[derive(Debug, Component, Default, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct CRGBA {
    pub value: [f32; 4]
}

impl CRGBA {
    pub fn to_tuple(&self) -> (f32, f32, f32, f32) {
        (
            self.value[0],
            self.value[1],
            self.value[2],
            self.value[3]
        )
    }
}
