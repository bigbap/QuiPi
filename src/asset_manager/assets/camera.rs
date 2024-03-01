use serde::{Deserialize, Serialize};

use crate::{
    prelude::qp_ecs::{
        components::{CTransform, CTransform2D},
        Component,
    },
    QPResult,
};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct OrthographicCameraParams {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for OrthographicCameraParams {
    fn default() -> Self {
        Self {
            left: 0.0,
            right: 800.0,
            bottom: 0.0,
            top: 600.0,
            near: 0.0,
            far: 0.2,
        }
    }
}

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
            view: glm::Mat4::identity(),
            params,
            zoom: 1.0,
            transform,
        };

        camera.projection = camera.calc_projection_matrix();
        camera.view = camera.calc_view_matrix();

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
            view: glm::Mat4::identity(),
            params,
            zoom,
            transform,
        };

        camera.projection = camera.calc_projection_matrix();
        camera.view = camera.calc_view_matrix();

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

    pub fn calc_view_matrix(&self) -> glm::Mat4 {
        let position = glm::vec3(self.transform.translate.x, self.transform.translate.y, 0.0);

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

// 3D camera

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct PerspectiveCameraParams {
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,

    pub front: glm::Vec3,
    pub up: glm::Vec3,
    pub right: glm::Vec3,

    world_up: glm::Vec3,
}

impl Default for PerspectiveCameraParams {
    fn default() -> Self {
        Self {
            fov: 45.0,
            aspect: 800.0 / 600.0,
            near: 0.1,
            far: 100.0,
            front: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 0.0, 0.0),
            right: glm::vec3(0.0, 0.0, 0.0),
            world_up: glm::vec3(0.0, 1.0, 0.0),
        }
    }
}

#[derive(Debug, Component, Serialize, Deserialize, PartialEq)]
pub struct RCamera3D {
    pub projection: glm::Mat4,
    pub view: glm::Mat4,
    pub params: PerspectiveCameraParams,
    pub transform: CTransform,
}

impl Default for RCamera3D {
    fn default() -> Self {
        let params = PerspectiveCameraParams::default();
        let transform = CTransform::default();

        let mut camera = Self {
            projection: glm::Mat4::identity(),
            view: glm::Mat4::identity(),
            params,
            transform,
        };

        camera.projection = camera.calc_projection_matrix();
        camera.view = camera.calc_view_matrix();

        camera
    }
}

impl RCamera3D {
    pub fn new(params: PerspectiveCameraParams, transform: CTransform) -> QPResult<Self> {
        let mut camera = Self {
            projection: glm::Mat4::identity(),
            view: glm::Mat4::identity(),
            params,
            transform,
        };

        camera.projection = camera.calc_projection_matrix();
        camera.view = camera.calc_view_matrix();

        Ok(camera)
    }

    pub fn calc_projection_matrix(&self) -> glm::Mat4 {
        let params = self.params;

        glm::perspective(params.aspect, params.fov, params.near, params.far)
    }

    pub fn calc_view_matrix(&self) -> glm::Mat4 {
        let position = glm::vec3(
            self.transform.translate.x,
            self.transform.translate.y,
            self.transform.translate.z,
        );

        glm::look_at(&position, &(position + self.params.front), &self.params.up)
    }

    pub fn update_vectors(&mut self) {
        self.params.right = glm::normalize(&glm::cross(&self.params.front, &self.params.world_up));
        self.params.up = glm::normalize(&glm::cross(&self.params.right, &self.params.front));
    }
}
