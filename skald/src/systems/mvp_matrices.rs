use crate::{
    Registry,
    VersionedIndex,
    components::{
        CTransform,
        CGizmo3D,
        CModelMatrix,
        CViewMatrix,
    }
};

/**
* set the model matrix for an entity.
*
* entity must have the following components:
* - CTransform
* - CModelMatrix
*/
pub fn s_set_model_matrix(
    entity: &VersionedIndex,
    registry: &mut Registry,
) {
    if let (Some(transform), Some(_)) = (
        registry.get_component::<CTransform>(entity),
        registry.get_component::<CModelMatrix>(entity)
    ) {
        let model = transform.to_matrix();
        let model_matrix = registry.get_component_mut::<CModelMatrix>(entity).unwrap();

        model_matrix.0 = model;
    }
}

/**
* set the view matrix for a camera.
*
* camera must have the following components:
* - CPosition
* - CGizmo3D
* - CViewMatrix
*/
pub fn s_set_view_matrix(
    camera: &VersionedIndex,
    registry: &mut Registry,
) {
    if let (Some(transform), Some(gizmo), Some(_)) = (
        registry.get_component::<CTransform>(camera),
        registry.get_component::<CGizmo3D>(camera),
        registry.get_component::<CViewMatrix>(camera)
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

        let view_matrix = registry.get_component_mut::<CViewMatrix>(camera).unwrap();
        view_matrix.0 = matrix;
    }
}
