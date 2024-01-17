use crate::Component;

#[derive(Debug, Component)]
pub struct CGizmo3D {
    pub front: glm::Vec3,
    pub up: glm::Vec3
}

impl CGizmo3D {
    pub fn right(&self) -> glm::Vec3 {
        glm::normalize(&glm::cross(&self.front, &self.up))
    }
}
