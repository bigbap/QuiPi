use crate::prelude::qp_ecs::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Serialize, Deserialize, PartialEq, Clone)]
pub struct CGizmo {
    pub front: glm::Vec3,
    pub up: glm::Vec3,
    pub right: glm::Vec3,

    pub world_up: glm::Vec3,
}

impl Default for CGizmo {
    fn default() -> Self {
        let mut gizmo = Self {
            front: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 0.0, 0.0),
            right: glm::vec3(0.0, 0.0, 0.0),
            world_up: glm::vec3(0.0, 1.0, 0.0),
        };

        gizmo.right = gizmo_right(&gizmo.front, &gizmo.world_up);
        gizmo.up = gizmo_up(&gizmo.front, &gizmo.right);

        gizmo
    }
}

pub fn gizmo_right(front: &glm::Vec3, world_up: &glm::Vec3) -> glm::Vec3 {
    glm::normalize(&glm::cross(front, world_up))
}

pub fn gizmo_up(front: &glm::Vec3, right: &glm::Vec3) -> glm::Vec3 {
    glm::normalize(&glm::cross(right, front))
}
