use crate::{
    prelude::{
        qp_ecs::components::{CDistance, CEulerAngles, CGizmo, CTarget, CTransform},
        GlobalRegistry, Index,
    },
    QPResult,
};

/**
* apply velocity vector to position
*
* requires the following components:
* - CGizmo3D
* - CPosition
*/
pub fn apply_velocity(
    registry: &mut GlobalRegistry,
    entity: &Index,
    delta: f32,
    velocity: glm::Vec3,
) -> QPResult<()> {
    if let (Some(gizmo), Some(_)) = (
        registry.entity_manager.get::<CGizmo>(entity),
        registry.entity_manager.get::<CTransform>(entity),
    ) {
        let mut change_vec = glm::vec3(0.0, 0.0, 0.0);

        change_vec += gizmo.front * velocity.z * delta;
        change_vec += gizmo.up * velocity.y * delta;
        change_vec += gizmo.right * velocity.x * delta;

        let transform = registry
            .entity_manager
            .get_mut::<CTransform>(entity)
            .unwrap();
        transform.translate.x += change_vec.x;
        transform.translate.y += change_vec.y;
        transform.translate.z += change_vec.z;
    }

    Ok(())
}

pub fn apply_follow_target(registry: &mut GlobalRegistry, entity: &Index) -> QPResult<()> {
    if let (Some(_), Some(distance), Some(target), Some(angles)) = (
        registry.entity_manager.get::<CTransform>(entity),
        registry.entity_manager.get::<CDistance>(entity),
        registry.entity_manager.get::<CTarget>(entity),
        registry.entity_manager.get::<CEulerAngles>(entity),
    ) {
        let pos = glm::vec3(
            target.x + distance.0 * angles.yaw.cos() * angles.pitch.sin(),
            target.y + distance.0 * angles.pitch.cos(),
            target.z + distance.0 * angles.yaw.sin() * angles.pitch.sin(),
        );

        let transform = registry
            .entity_manager
            .get_mut::<CTransform>(entity)
            .unwrap();
        transform.translate.x = pos.x;
        transform.translate.y = pos.y;
        transform.translate.z = pos.z;
    }

    Ok(())
}
