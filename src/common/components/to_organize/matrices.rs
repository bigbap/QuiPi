use quipi_core::{Component, GlobalRegistry, Index};

use super::{CGizmo, CTransform};

#[derive(Debug, Component, PartialEq)]
pub struct CModelMatrix(pub glm::Mat4);

impl Default for CModelMatrix {
    fn default() -> Self {
        Self(glm::Mat4::identity())
    }
}

impl CModelMatrix {
    pub fn update_model_matrix(entity: &Index, registry: &mut GlobalRegistry) {
        let Some(transform) = registry.entity_manager.get::<CTransform>(entity) else {
            return;
        };
        let matrix = transform.to_matrix();

        if let Some(model) = registry.entity_manager.get_mut::<CModelMatrix>(entity) {
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
    pub fn update_view_matrix(camera: &Index, registry: &mut GlobalRegistry) {
        if let (Some(transform), Some(gizmo)) = (
            registry.entity_manager.get::<CTransform>(camera),
            registry.entity_manager.get::<CGizmo>(camera),
        ) {
            let position = glm::vec3(
                transform.translate.x,
                transform.translate.y,
                transform.translate.z,
            );

            let matrix = glm::look_at(&position, &(position + gizmo.front), &gizmo.up);

            if let Some(view) = registry.entity_manager.get_mut::<CViewMatrix>(camera) {
                view.0 = matrix;
            }
        }
    }
}
