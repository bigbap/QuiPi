use crate::Component;

#[derive(Debug, Component)]
pub struct CGizmo3D {
    pub front: glm::Vec3,
    pub up: glm::Vec3,
    pub right: glm::Vec3,

    world_up: glm::Vec3
}

impl CGizmo3D {
    pub fn new(
        front: glm::Vec3,
        world_up: glm::Vec3
    ) -> Self {
        let mut gizmo = Self {
            front,
            up: glm::vec3(0.0, 0.0, 0.0),
            right: glm::vec3(0.0, 0.0, 0.0),
            world_up
        };

        gizmo.update_vectors();
        gizmo
    }
    
    pub fn update_vectors(&mut self) {
        self.right = glm::normalize(&glm::cross(&self.front, &self.world_up));
        self.up = glm::normalize(&glm::cross(&self.right, &self.front));
    }
}
