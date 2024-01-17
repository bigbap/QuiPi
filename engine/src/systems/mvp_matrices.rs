use crate::{
    Registry,
    VersionedIndex,
    components::{
        CTransform,
        CViewSettings,
        CZPlanes,
        CGizmo3D,
        CPosition
    }
};

pub fn s_model_matrix_3d(
    entity: &VersionedIndex,
    registry: &Registry,
) -> glm::Mat4 {
    let model = glm::Mat4::identity();
    let Some(transform) = registry.get_component::<CTransform>(entity) else { return model };
    
    let model = match transform.translate {
        Some(translate) => glm::translate(&model, &translate),
        None => model
    };
    let model = match transform.rotate {
        None => model,
        Some(rotate) => glm::rotate(&model, transform.angle, &glm::normalize(&rotate))
    };
    match transform.scale {
        Some(scale) => glm::scale(&model, &scale),
        None => model
    }
}

pub fn s_projection_matrix_3d(
    camera: &VersionedIndex,
    registry: &Registry,
) -> glm::Mat4 {
    let view = registry.get_component::<CViewSettings>(camera).unwrap();
    let z_planes = registry.get_component::<CZPlanes>(camera).unwrap();

    glm::perspective(
        view.fov.to_radians(),
        view.aspect_ratio,
        z_planes.near_plane,
        z_planes.far_plane
    )
}

pub fn s_ortho_projection_matrix_3d(
    camera: &VersionedIndex,
    registry: &Registry,
    width: f32,
    height: f32
) -> glm::Mat4 {
    let z_planes = registry.get_component::<CZPlanes>(camera).unwrap();

    glm::ortho(
        0.0,
        width,
        0.0,
        height,
        z_planes.near_plane,
        z_planes.far_plane
    )
}

pub fn s_view_matrix_3d(
    camera: &VersionedIndex,
    registry: &Registry,
) -> glm::Mat4 {
    let position = registry.get_component::<CPosition>(camera).unwrap();
    let gizmo = registry.get_component::<CGizmo3D>(camera).unwrap();

    let position = glm::vec3(position.x, position.y, position.z);

    glm::look_at(
        &position, 
        &(position + gizmo.front),
        &gizmo.up
    )
}
