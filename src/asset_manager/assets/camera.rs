use serde::{Serialize, Deserialize};

use crate::{
    QPResult,
    prelude::{
        qp_data::OrthographicCameraParams,
        qp_ecs::{
            components::CTransform2D,
            Component
        }
    }
};

#[derive(Debug, Component, Serialize, Deserialize, PartialEq)]
pub struct RCamera2D {
    pub projection: glm::Mat4,
    pub view: glm::Mat4,

    pub params: OrthographicCameraParams,
    pub transform: CTransform2D,
}

impl Default for RCamera2D {
    fn default() -> Self {
        let params = OrthographicCameraParams::default();
        let transform = CTransform2D::default();
        Self {
            projection: RCamera2D::calc_projection_matrix(&params),
            view: RCamera2D::calc_view_matrix(&transform),
            params,
            transform
        }
    }
}

impl RCamera2D {
    pub fn new(
        params: OrthographicCameraParams,
        transform: CTransform2D,
    ) -> QPResult<Self> {
        let camera = Self {
            projection: RCamera2D::calc_projection_matrix(&params),
            view: RCamera2D::calc_view_matrix(&transform),
            params,
            transform,
        };

        Ok(camera)
    }

    pub fn calc_projection_matrix(params: &OrthographicCameraParams) -> glm::Mat4 {
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
