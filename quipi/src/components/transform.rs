use serde::{Serialize, Deserialize};

use crate::systems::ec_store::Component;

/**
* the model portion for a Model View Projection matrix
*/
#[derive(Debug, Component, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct CTransform {
    pub translate: glm::Vec3,
    pub rotate: Option<glm::Vec3>,
    pub scale: Option<glm::Vec3>,

    pub angle: f32,
}

impl Default for CTransform {
    fn default() -> Self {
        Self {
            translate: glm::vec3(0.0, 0.0, 0.0),
            rotate: None,
            scale: None,

            angle: 0.0
        }
    }
}

impl CTransform {
    /**
    * Transformations do not commute, so it's important to know
    * the order that is being used here.
    *
    * Transformations are done in this order:
    * 1. translate
    * 2. rotate
    * 3. scale
    */
    pub fn to_matrix(&self) -> glm::Mat4 {
        let matrix = glm::Mat4::identity();
        let matrix = glm::translate(&matrix, &self.translate);
        let matrix = match &self.rotate {
            None => matrix,
            Some(rotate) => {
                // TODO: change this to use quaternions.
                glm::rotate(&matrix, self.angle, &glm::normalize(rotate))
            }
        };
        match self.scale {
            Some(scale) => glm::scale(&matrix, &scale),
            None => matrix
        }
    }
}

/**
* 3D direction vector
*/
#[derive(Debug, Component, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct CDirection {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

/**
* https://en.wikipedia.org/wiki/Euler_angles
*/
#[derive(Debug, Component, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct CEulerAngles {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32
}
