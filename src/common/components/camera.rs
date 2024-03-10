use crate::prelude::qp_ecs::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Serialize, Deserialize, PartialEq, Clone)]
pub struct COrthographic {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for COrthographic {
    fn default() -> Self {
        Self {
            left: 0.0,
            right: 800.0,
            bottom: 0.0,
            top: 600.0,
            near: 0.0,
            far: 0.2,
        }
    }
}

pub struct CPerspective {
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for CPerspective {
    fn default() -> Self {
        Self {
            fov: 45.0,
            aspect: 800.0 / 600.0,
            near: 0.1,
            far: 100.0,
        }
    }
}
