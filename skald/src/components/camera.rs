use crate::Component;

#[derive(Debug, Component)]
pub struct CCamera {
    pub projection_matrix: glm::Mat4,
}

impl CCamera {
    pub fn new_orthographic(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            projection_matrix: glm::ortho(
                left,
                right,
                bottom,
                top,
                near,
                far
            ),
        })
    }

    pub fn new_perspective(
        aspect: f32,
        fov: f32,
        near: f32,
        far: f32
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            projection_matrix: glm::perspective(
                aspect,
                fov,
                near,
                far
            )
        })
    }
}
