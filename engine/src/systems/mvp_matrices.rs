use crate::{
    Registry,
    VersionedIndex,
    components::{
        CTransform,
        CViewSettings,
        CZPlanes,
        CGizmo3D,
        CPosition,
        CDimensions,
        CModelMatrix,
        CViewMatrix,
        CProjectionMatrix, CTarget, distance
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
        let model = glm::Mat4::identity();
        let model = match transform.translate {
            Some(translate) => glm::translate(&model, &translate),
            None => model
        };
        let model = match &transform.rotate {
            None => model,
            Some(rotate) => {
                for (axis, angle) in rotate {
                    glm::rotate(&model, *angle, &glm::normalize(axis));
                }
                model
            }
        };
        let model = match transform.scale {
            Some(scale) => glm::scale(&model, &scale),
            None => model
        };

        let model_matrix = registry.get_component_mut::<CModelMatrix>(entity).unwrap();
        model_matrix.0 = model;
    }
}

/**
* set the perspective projection matrix for a camera.
*
* camera must have the following components:
* - CViewSettings
* - CZPlanes
* - CProjectionMatrix
*/
pub fn s_set_projection_matrix(
    camera: &VersionedIndex,
    registry: &mut Registry,
) {
    if let (Some(view), Some(z_planes), Some(_)) = (
        registry.get_component::<CViewSettings>(camera),
        registry.get_component::<CZPlanes>(camera),
        registry.get_component::<CProjectionMatrix>(camera)
    ) {
        let matrix = glm::perspective(
            view.fov.to_radians(),
            view.aspect_ratio,
            z_planes.near_plane,
            z_planes.far_plane
        );

        let projection_matrix = registry.get_component_mut::<CProjectionMatrix>(camera).unwrap();
        projection_matrix.0 = matrix;
    }
}

/**
* set the orthographic projection matrix for a camera.
*
* camera must have the following components:
* - CPosition
* - CZPlanes
* - CDimensions
* - CProjectionMatrix
*/
pub fn s_set_ortho_projection_matrix(
    camera: &VersionedIndex,
    registry: &mut Registry
) {
    if let (Some(position), Some(z_planes), Some(dimensions), Some(_)) = (
        registry.get_component::<CPosition>(camera),
        registry.get_component::<CZPlanes>(camera),
        registry.get_component::<CDimensions>(camera),
        registry.get_component::<CProjectionMatrix>(camera)
    ) {
        let pos_x = position.x;
        let pos_y = position.y;
        let w = dimensions.width;
        let h = dimensions.height;
        let matrix = glm::ortho(
            pos_x,
            pos_x + w,
            pos_y,
            pos_y + h,
            z_planes.near_plane,
            z_planes.far_plane
        );

        let projection_matrix = registry.get_component_mut::<CProjectionMatrix>(camera).unwrap();
        projection_matrix.0 = matrix;
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
    if let (Some(position), Some(target), Some(gizmo), Some(_)) = (
        registry.get_component::<CPosition>(camera),
        registry.get_component::<CTarget>(camera),
        registry.get_component::<CGizmo3D>(camera),
        registry.get_component::<CViewMatrix>(camera)
    ) {
        let position = glm::vec3(position.x, position.y, position.z);
        let target = glm::vec3(target.x, target.y, target.z);
        let distance = target - position;

        let matrix = glm::look_at(
            &position, 
            &(position + gizmo.front),
            &gizmo.up
        );

        let view_matrix = registry.get_component_mut::<CViewMatrix>(camera).unwrap();
        view_matrix.0 = matrix;
    }
}
