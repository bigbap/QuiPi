use crate::{Component, Registry, VersionedIndex};

use super::{CTransform, CGizmo3D};

#[derive(Debug, Component, PartialEq)]
pub struct CModelMatrix(pub glm::Mat4);

impl Default for CModelMatrix {
    fn default() -> Self {
        Self(glm::Mat4::identity())
    }
}

impl CModelMatrix {
    pub fn update_model_matrix(
        &mut self,
        matrix: glm::Mat4,
    ) {
        self.0 = matrix;
    }
}

#[derive(Debug, Component, PartialEq)]
pub struct CViewMatrix(pub glm::Mat4);

impl Default for CViewMatrix {
    fn default() -> Self {
        Self(glm::Mat4::identity())
    }
}

impl CViewMatrix {
    pub fn update_view_matrix(
        camera: &VersionedIndex,
        registry: &mut Registry
    ) {
        if let (Some(transform), Some(gizmo)) = (
            registry.entities.get::<CTransform>(camera),
            registry.entities.get::<CGizmo3D>(camera),
        ) {
            let position = glm::vec3(
                transform.translate.x,
                transform.translate.y,
                transform.translate.z
            );

            let matrix = glm::look_at(
                &position, 
                &(position + gizmo.front),
                &gizmo.up
            );

            if let Some(view) = registry.entities.get_mut::<CViewMatrix>(camera) {
                view.0 = matrix;
            }
        }
    }
}
