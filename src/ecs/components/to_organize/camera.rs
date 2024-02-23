use quipi_core::Component;
use serde::{Serialize, Deserialize};


#[derive(Debug, Component, Serialize, Deserialize, PartialEq)]
pub struct CCamera {
    pub projection: glm::Mat4,
    pub params: CameraParams
}

impl Default for CCamera {
    fn default() -> Self {
        let params = CameraParams::default();
        Self {
            projection: CCamera::calc_projection_matrix(&params),
            params
        }
    }
}

impl CCamera {
    pub fn new(params: CameraParams) -> Result<Self, Box<dyn std::error::Error>> {
        let camera = Self {
            projection: CCamera::calc_projection_matrix(&params),
            params
        };

        Ok(camera)
    }

    pub fn calc_projection_matrix(params: &CameraParams) -> glm::Mat4 {
        glm::perspective(
            params.aspect,
            params.fov,
            params.near,
            params.far
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct CameraParams {
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32
}

impl Default for CameraParams {
    fn default() -> Self {
        Self {
            fov: 45.0,
            aspect: 800.0/600.0,
            near: 0.1,
            far: 100.0
        }
    }
}
