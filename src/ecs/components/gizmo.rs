use super::super::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Serialize, Deserialize, PartialEq)]
pub struct CGizmo {
    pub front: glm::Vec3,
    pub up: glm::Vec3,
    pub right: glm::Vec3,

    world_up: glm::Vec3,
}

impl Default for CGizmo {
    fn default() -> Self {
        let mut gizmo = Self {
            front: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 0.0, 0.0),
            right: glm::vec3(0.0, 0.0, 0.0),
            world_up: glm::vec3(0.0, 1.0, 0.0),
        };

        gizmo.update_vectors();

        gizmo
    }
}

impl CGizmo {
    pub fn update_vectors(&mut self) {
        self.right = glm::normalize(&glm::cross(&self.front, &self.world_up));
        self.up = glm::normalize(&glm::cross(&self.right, &self.front));
    }
}
