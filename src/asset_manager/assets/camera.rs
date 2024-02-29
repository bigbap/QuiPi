use serde::{Deserialize, Serialize};

use crate::{
    prelude::{
        qp_data::OrthographicCameraParams,
        qp_ecs::{components::CTransform2D, Component},
    },
    QPResult,
};

#[derive(Debug, Component, Serialize, Deserialize, PartialEq)]
pub struct RCamera2D {
    pub projection: glm::Mat4,
    pub view: glm::Mat4,

    pub params: OrthographicCameraParams,
    pub zoom: f32,
    pub transform: CTransform2D,
}

impl Default for RCamera2D {
    fn default() -> Self {
        let params = OrthographicCameraParams::default();
        let transform = CTransform2D::default();

        let mut camera = Self {
            projection: glm::Mat4::identity(),
            view: RCamera2D::calc_view_matrix(&transform),
            params,
            zoom: 1.0,
            transform,
        };

        camera.projection = camera.calc_projection_matrix();

        camera
    }
}

impl RCamera2D {
    pub fn new(
        params: OrthographicCameraParams,
        zoom: f32,
        transform: CTransform2D,
    ) -> QPResult<Self> {
        let mut camera = Self {
            projection: glm::Mat4::identity(),
            view: RCamera2D::calc_view_matrix(&transform),
            params,
            zoom,
            transform,
        };

        camera.projection = camera.calc_projection_matrix();

        Ok(camera)
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;

        self.projection = self.calc_projection_matrix();
    }

    pub fn calc_projection_matrix(&self) -> glm::Mat4 {
        let params = self.params();

        glm::ortho(
            params.left,
            params.right,
            params.bottom,
            params.top,
            params.near,
            params.far,
        )
    }

    pub fn calc_view_matrix(transform: &CTransform2D) -> glm::Mat4 {
        let position = glm::vec3(transform.translate.x, transform.translate.y, 0.0);

        glm::look_at(
            &position,
            &(position + glm::vec3(0.0, 0.0, -1.0)),
            &glm::vec3(0.0, 1.0, 0.0),
        )
    }

    pub fn params(&self) -> OrthographicCameraParams {
        let zoom_x = (self.params.right - self.params.left) / self.zoom;
        let zoom_y = (self.params.top - self.params.bottom) / self.zoom;

        OrthographicCameraParams {
            left: self.params.left + zoom_x,
            right: self.params.right - zoom_x,
            bottom: self.params.bottom + zoom_y,
            top: self.params.top - zoom_y,
            near: self.params.near,
            far: self.params.far,
        }
    }
}
