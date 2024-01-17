use crate::{
    Registry,
    VersionedIndex,
    components::{
        CPosition,
        CGizmo3D
    }
};

pub fn s_apply_velocity(
    registry: &mut Registry,
    camera: &VersionedIndex,
    delta: f32,
    velocity: glm::Vec3
) -> Result<(), Box<dyn std::error::Error>> {
    let Some(gizmo) = registry.get_component::<CGizmo3D>(camera) else { return Ok(()) };

    let mut new_pos = glm::vec3(0.0, 0.0, 0.0);
    
    new_pos += gizmo.front * velocity.z * delta;
    new_pos += gizmo.up * velocity.y * delta;
    new_pos += gizmo.right() * velocity.x * delta;

    let Some(position) = registry.get_component_mut::<CPosition>(camera) else { return Ok(()) };
    position.x += new_pos.x;
    position.y += new_pos.y;
    position.z += new_pos.z;

    Ok(())
}

