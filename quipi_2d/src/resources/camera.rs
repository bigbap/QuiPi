use serde::{Serialize, Deserialize};

use quipi_core::Component;

use crate::components::{CTransform2D, CVelocity2D};

#[derive(Debug, Component, Serialize, Deserialize, PartialEq)]
pub struct RCamera2D {
    pub projection: glm::Mat4,
    pub view: glm::Mat4,

    pub params: CameraParams,
    pub transform: CTransform2D,
    pub velocity: CVelocity2D,
}

impl Default for RCamera2D {
    fn default() -> Self {
        let params = CameraParams::default();
        let transform = CTransform2D::default();
        Self {
            projection: RCamera2D::calc_projection_matrix(&params),
            view: RCamera2D::calc_view_matrix(&transform),
            params,
            transform,
            velocity: CVelocity2D::default()
        }
    }
}

impl RCamera2D {
    pub fn new(
        params: CameraParams,
        transform: CTransform2D,
        velocity: CVelocity2D
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let camera = Self {
            projection: RCamera2D::calc_projection_matrix(&params),
            view: RCamera2D::calc_view_matrix(&transform),
            params,
            transform,
            velocity
        };

        Ok(camera)
    }

    pub fn calc_projection_matrix(params: &CameraParams) -> glm::Mat4 {
        glm::ortho(
            params.left,
            params.right,
            params.bottom,
            params.top,
            params.near,
            params.far
        )
    }

    pub fn calc_view_matrix(transform: &CTransform2D) -> glm::Mat4 {
        let position = glm::vec3(
            transform.translate.x,
            transform.translate.y,
            0.0
        );

        glm::look_at(
            &position, 
            &(position + glm::vec3(0.0, 0.0, -1.0)),
            &glm::vec3(0.0, 1.0, 0.0)
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct CameraParams {
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
            left: 0.0,
            right: 800.0,
            bottom: 0.0,
            top: 600.0,
            near: 0.0,
            far: 0.2
        }
    }
}
