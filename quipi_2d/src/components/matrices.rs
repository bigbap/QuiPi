use quipi_core::{Component, Registry, VersionedIndex};

use super::CTransform2D;

#[derive(Debug, Component, PartialEq)]
pub struct CModelMatrix2D(pub glm::Mat4);

impl Default for CModelMatrix2D {
    fn default() -> Self {
        Self(glm::Mat4::identity())
    }
}

impl CModelMatrix2D {
    pub fn update_model_matrix(
        entity: &VersionedIndex,
        registry: &mut Registry
    ) {
        let Some(transform) = registry.entities.get::<CTransform2D>(entity) else { return; };
        let matrix = transform.to_matrix();

        if let Some(model) = registry.entities.get_mut::<CModelMatrix2D>(entity) {
            model.0 = matrix;
        }
    }
}

#[derive(Debug, Component, PartialEq)]
pub struct CViewMatrix2D(pub glm::Mat4);

impl Default for CViewMatrix2D {
    fn default() -> Self {
        Self(glm::Mat4::identity())
    }
}

impl CViewMatrix2D {
    pub fn update_view_matrix(
        camera: &VersionedIndex,
        registry: &mut Registry
    ) {
        if let Some(transform) = registry.entities.get::<CTransform2D>(camera) {
            let position = glm::vec3(
                transform.translate.x,
                transform.translate.y,
                0.0
            );

            let matrix = glm::look_at(
                &position, 
                &(position + glm::vec3(0.0, 0.0, -1.0)),
                &glm::vec3(0.0, 1.0, 0.0)
            );

            if let Some(view) = registry.entities.get_mut::<CViewMatrix2D>(camera) {
                view.0 = matrix;
            }
        }
    }
}