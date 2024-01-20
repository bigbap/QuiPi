use crate::{
    Registry,
    VersionedIndex,
    components::{
        CPosition,
        CGizmo3D
    }
};

/**
* apply velocity vector to position
*
* requires the following components:
* - CGizmo3D
* - CPosition
*/
pub fn s_apply_velocity(
    registry: &mut Registry,
    entity: &VersionedIndex,
    delta: f32,
    velocity: glm::Vec3
) -> Result<(), Box<dyn std::error::Error>> {
    if let (Some(gizmo), Some(_)) = (
        registry.get_component::<CGizmo3D>(entity),
        registry.get_component::<CPosition>(entity)
    ) {
        let mut change_vec = glm::vec3(0.0, 0.0, 0.0);

        change_vec += gizmo.front * velocity.z * delta;
        change_vec += gizmo.up * velocity.y * delta;
        change_vec += gizmo.right() * velocity.x * delta;

        let position = registry.get_component_mut::<CPosition>(entity).unwrap();
        position.x += change_vec.x;
        position.y += change_vec.y;
        position.z += change_vec.z;
    }

    Ok(())
}

