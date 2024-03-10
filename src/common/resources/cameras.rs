use crate::prelude::qp_common::components::*;
use crate::resources::Resource;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub(crate) struct CameraList {
    pub cameras: HashMap<CameraId, Box<dyn Camera>>,
}

impl CameraList {
    pub fn add_camera<C: Camera + 'static>(&mut self, id: u64, camera: C) -> CameraId {
        let id = CameraId(id);

        self.cameras.insert(id, Box::new(camera));

        id
    }

    pub fn get<C: Camera + 'static>(&self, id: &CameraId) -> Option<&C> {
        match self.cameras.get(id) {
            Some(camera) => camera.as_any().downcast_ref::<C>(),
            _ => None,
        }
    }

    pub fn get_mut<C: Camera + 'static>(&mut self, id: &CameraId) -> Option<&mut C> {
        match self.cameras.get_mut(id) {
            Some(camera) => camera.as_any_mut().downcast_mut::<C>(),
            _ => None,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct CameraId(pub u64);

pub trait Camera {
    fn view_matrix(&self) -> glm::Mat4;

    fn projection_matrix(&self) -> glm::Mat4;

    fn as_any(&self) -> &dyn std::any::Any;

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

#[derive(Default, Clone)]
pub struct Camera2D {
    pub orthographic: COrthographic,
    pub transform: CTransform2D,
    pub view: CViewMatrix,
    pub projection: CProjectionMatrix,
}

impl Camera for Camera2D {
    fn view_matrix(&self) -> glm::Mat4 {
        let position = glm::vec3(self.transform.translate.x, self.transform.translate.y, 0.0);

        glm::look_at(
            &position,
            &(position + glm::vec3(0.0, 0.0, -1.0)),
            &glm::vec3(0.0, 1.0, 0.0),
        )
    }

    fn projection_matrix(&self) -> glm::Mat4 {
        glm::ortho(
            self.orthographic.left,
            self.orthographic.right,
            self.orthographic.bottom,
            self.orthographic.top,
            self.orthographic.near,
            self.orthographic.far,
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Default)]
pub struct Camera3D {
    pub perspective: CPerspective,
    pub transform: CTransform,
    pub view: CViewMatrix,
    pub projection: CProjectionMatrix,
    pub gizmo: CGizmo,
}

impl Camera for Camera3D {
    fn view_matrix(&self) -> glm::Mat4 {
        let position = glm::vec3(
            self.transform.translate.x,
            self.transform.translate.y,
            self.transform.translate.z,
        );

        glm::look_at(&position, &(position + self.gizmo.front), &self.gizmo.up)
    }

    fn projection_matrix(&self) -> glm::Mat4 {
        glm::perspective(
            self.perspective.aspect,
            self.perspective.fov,
            self.perspective.near,
            self.perspective.far,
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
