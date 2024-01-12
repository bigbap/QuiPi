use crate::Component;

#[derive(Debug)]
pub enum CameraProjection {
    Perspective,
    Orthographic(f32, f32)
}

#[derive(Debug, Component)]
pub struct Camera3D {
    pub projection: CameraProjection,
    pub pos: glm::Vec3,
    pub front: glm::Vec3,
    pub up: glm::Vec3,
    
    pub fov: f32,
    pub aspect_ratio: f32,
    pub pitch: f32,
    pub yaw: f32,

    pub near_plane: f32,
    pub far_plane: f32
}

impl Camera3D {
    pub fn position(&self) -> (f32, f32, f32) {
        (self.pos[0], self.pos[1], self.pos[2])
    }

    pub fn direction(&self) -> (f32, f32, f32) {
        (self.front[0], self.front[1], self.front[2])
    }

    pub fn look_at(&self) -> glm::Mat4 {
        glm::look_at(
            &self.pos, 
            &(self.pos + self.front),
            &self.up)
    }

    pub fn projection(&self) -> glm::Mat4 {
        match self.projection {
            CameraProjection::Perspective => {
                glm::perspective(
                    self.fov.to_radians(),
                    self.aspect_ratio,
                    self.near_plane,
                    self.far_plane
                )
            },
            CameraProjection::Orthographic(width, height) => {
                glm::ortho(
                    0.0,
                    width,
                    0.0,
                    height,
                    self.near_plane,
                    self.far_plane
                )
            }
        }
    }

    pub fn right_vector(&self) -> glm::Vec3 {
        glm::normalize(&glm::cross(&self.front, &self.up))
    }
}

#[derive(Debug, Component)]
pub struct Camera2D {
    pub pos: glm::Vec2,
}

impl Camera2D {
    pub fn position(&self) -> (f32, f32, f32) {
        (self.pos[0], self.pos[1], 0.0)
    }
}
