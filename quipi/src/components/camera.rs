use crate::Component;

#[derive(Debug, Clone, Copy)]
pub struct PerspectiveParams {
    pub aspect: f32,
    pub fov: f32,
    pub near: f32,
    pub far: f32
}

#[derive(Debug, Clone, Copy)]
pub struct OrthographicParams {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum CameraParams {
    Perspective(PerspectiveParams),
    Orthographic(OrthographicParams)
}

#[derive(Debug, Component)]
pub struct CCamera {
    pub projection: glm::Mat4
}

impl Default for CCamera {
    fn default() -> Self {
        Self {
            projection: glm::identity()
        }
    }
}

impl CCamera {
    pub fn new(params: &CameraParams) -> Result<Self, Box<dyn std::error::Error>> {
        let mut camera = Self::default();

        camera.update_projection_matrix(params);

        Ok(camera)
    }

    pub fn update_projection_matrix(&mut self, params: &CameraParams) {
        self.projection = match params {
            CameraParams::Perspective(params) => {
                glm::perspective(params.aspect, params.fov, params.near, params.far)
            },
            CameraParams::Orthographic(params) => {
                glm::ortho(
                    params.left,
                    params.right,
                    params.bottom,
                    params.top,
                    params.near,
                    params.far
                )
            }
        };
    }
}
