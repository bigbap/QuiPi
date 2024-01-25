use crate::{
    Registry,
    VersionedIndex,
    components::{
        CPosition,
        CGizmo3D, CTarget, CEulerAngles, CDistance
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
        change_vec += gizmo.right * velocity.x * delta;

        let position = registry.get_component_mut::<CPosition>(entity).unwrap();
        position.x += change_vec.x;
        position.y += change_vec.y;
        position.z += change_vec.z;
    }

    Ok(())
}

pub fn s_apply_follow_target(
    registry: &mut Registry,
    entity: &VersionedIndex
) -> Result<(), Box<dyn std::error::Error>> {
    if let (Some(_), Some(distance), Some(target), Some(angles)) = (
        registry.get_component::<CPosition>(entity),
        registry.get_component::<CDistance>(entity),
        registry.get_component::<CTarget>(entity),
        registry.get_component::<CEulerAngles>(entity),
    ) {
        let pos = glm::vec3(
            target.x + distance.0 * angles.yaw.cos() * angles.pitch.sin(),
            target.y + distance.0 * angles.pitch.cos(),
            target.z + distance.0 * angles.yaw.sin() * angles.pitch.sin()
        );

        let position = registry.get_component_mut::<CPosition>(entity).unwrap();
        position.x = pos.x;
        position.y = pos.y;
        position.z = pos.z;
    }

    Ok(())

}
