use quipi_core::{Component, Registry, VersionedIndex};

use super::{CTransform, CGizmo};

#[derive(Debug, Component, PartialEq)]
pub struct CModelMatrix(pub glm::Mat4);

impl Default for CModelMatrix {
    fn default() -> Self {
        Self(glm::Mat4::identity())
    }
}

impl CModelMatrix {
    pub fn update_model_matrix(
        entity: &VersionedIndex,
        registry: &mut Registry
    ) {
        let Some(transform) = registry.entities.get::<CTransform>(entity) else { return; };
        let matrix = transform.to_matrix();

        if let Some(model) = registry.entities.get_mut::<CModelMatrix>(entity) {
            model.0 = matrix;
        }
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
            registry.entities.get::<CGizmo>(camera),
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
