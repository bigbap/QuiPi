use crate::Component;

#[derive(Debug, Default)]
pub enum CameraProjection {
    #[default] Perspective,
    Orthographic(f32, f32)
}

#[derive(Debug, Component)]
pub struct Camera3D {
    pub projection: CameraProjection,

    pub position: glm::Vec3,
    pub front: glm::Vec3,
    pub up: glm::Vec3,
    
    pub fov: f32,
    pub aspect_ratio: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub max_pitch: f32,
    pub min_pitch: f32,

    pub near_plane: f32,
    pub far_plane: f32,

    pub move_up: bool,
    pub move_down: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub move_forward: bool,
    pub move_backward: bool,
}

impl Default for Camera3D {
    fn default() -> Self {
        Camera3D {
            projection: CameraProjection::default(),
            position: glm::vec3(0.0, 0.0, 0.0),
            front: glm::vec3(0.0, 0.0, -1.0), // direction from camera to target
            up: glm::vec3(0.0, 1.0, 0.0),

            fov: 75.0,
            aspect_ratio: 0.0,
            pitch: 0.0,
            yaw: 90.0,
            max_pitch: 89.0,
            min_pitch: -89.0,

            near_plane: 0.1,
            far_plane: 100.0,

            move_up: false,
            move_down: false,
            move_left: false,
            move_right: false,
            move_forward: false,
            move_backward: false,
        }
    }
}

impl Camera3D {
    pub fn position_tup(&self) -> (f32, f32, f32) { (self.position.x, self.position.y, self.position.z) }
    pub fn front_tup(&self) -> (f32, f32, f32) { (self.front.x, self.front.y, self.front.z) }
    pub fn direction_tup(&self) -> (f32, f32, f32) { (-self.front.x, -self.front.y, -self.front.z) }

    pub fn right(&self) -> glm::Vec3 {
        glm::normalize(&glm::cross(&self.front, &self.up))
    }

    pub fn get_view(&self) -> glm::Mat4 {
        glm::look_at(
            &self.position, 
            &(self.position + self.front),
            &self.up
        )
    }

    pub fn get_projection(&self) -> glm::Mat4 {
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

    pub fn rotate(&mut self, x_offset: f32, y_offset: f32) {
        self.pitch = (self.pitch + y_offset).clamp(self.min_pitch, self.max_pitch);
        self.yaw += x_offset;

        self.front = glm::normalize(&-glm::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos()
        ))
    }

    pub fn apply_move(&mut self, speed: f32) {
        let mut new_pos = self.position;
        if self.move_forward {
            new_pos += self.front * speed;
        }
        if self.move_backward {
            new_pos -= self.front * speed;
        }
        if self.move_left {
            new_pos -= self.right() * speed;
        }
        if self.move_right {
            new_pos += self.right() * speed;
        }
        if self.move_up {
            new_pos -= self.up * speed;
        }
        if self.move_down {
            new_pos += self.up * speed;
        }

        // TODO: normalise the distance in case the movement is diagonal
        
        self.position = new_pos;
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub struct Camera2D {
    pub position: glm::Vec2
}

impl Default for Camera2D {
    fn default() -> Self {
        Self {
            position: glm::vec2(0.0, 0.0)
        }
    }
}
