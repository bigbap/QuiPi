use serde::{Serialize, Deserialize};

use crate::{
    Component,
    schemas::camera::{
        CameraParams,
        CameraKind
    },
};


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

