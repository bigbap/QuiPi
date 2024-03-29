use serde::{Deserialize, Serialize};

use super::super::prelude::Component;

/**
* https://en.wikipedia.org/wiki/Euler_angles
*/
#[derive(Debug, Component, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct CEulerAngles {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32
}