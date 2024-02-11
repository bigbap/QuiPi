use serde::{Serialize, Deserialize};

use crate::Component;


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
        match params.kind {
            CameraKind::Cam2D => glm::ortho(
                params.left,
                params.right,
                params.bottom,
                params.top,
                params.near,
                params.far
            ),
            CameraKind::Cam3D => glm::perspective(
                params.aspect,
                params.fov,
                params.near,
                params.far
            )
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum CameraKind {
    Cam3D,
    Cam2D
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct CameraParams {
    pub kind: CameraKind,
    pub fov: f32,
    pub aspect: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32
}

impl Default for CameraParams {
    fn default() -> Self {
        Self {
            kind: CameraKind::Cam2D,
            fov: 0.0,
            aspect: 0.0,
            left: 0.0,
            right: 800.0,
            bottom: 0.0,
            top: 600.0,
            near: 0.0,
            far: 0.2
        }
    }
}
