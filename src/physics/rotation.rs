use crate::prelude::{
    qp_ecs::components::{CEulerAngles, CGizmo},
    GlobalRegistry, Index,
};

pub fn s_rotate_camera(registry: &mut GlobalRegistry, camera: &Index) {
    let Some(euler_angles) = registry.entities.get::<CEulerAngles>(camera) else {
        return;
    };

    let front = glm::normalize(&-glm::vec3(
        euler_angles.yaw.to_radians().cos() * euler_angles.pitch.to_radians().cos(),
        euler_angles.pitch.to_radians().sin(),
        euler_angles.yaw.to_radians().sin() * euler_angles.pitch.to_radians().cos(),
    ));

    let Some(gizmo) = registry.entities.get_mut::<CGizmo>(camera) else {
        return;
    };
    gizmo.front = front;
    gizmo.update_vectors();
}

pub fn s_update_angles(
    registry: &mut GlobalRegistry,
    camera: &Index,
    x_offset: f32,
    y_offset: f32,
    min_pitch: f32,
    max_pitch: f32,
) -> Option<CEulerAngles> {
    let euler_angle = match registry.entities.get_mut::<CEulerAngles>(camera) {
        Some(val) => val,
        _ => return None,
    };
    euler_angle.pitch = (euler_angle.pitch + y_offset).clamp(min_pitch, max_pitch);
    euler_angle.yaw += x_offset;

    Some(CEulerAngles {
        pitch: euler_angle.pitch,
        yaw: euler_angle.yaw,
        ..CEulerAngles::default()
    })
}
