use crate::{
    Registry,
    VersionedIndex,
    components::{
        CEulerAngles,
        CGizmo3D
    }
};

pub fn s_rotate(
    registry: &mut Registry,
    camera: &VersionedIndex,
    euler_angles: CEulerAngles
) {
    let gizmo = match registry.get_component_mut::<CGizmo3D>(camera) {
        Some(val) => val,
        _ => return
    };


    gizmo.front = glm::normalize(&-glm::vec3(
        euler_angles.yaw.to_radians().cos() * euler_angles.pitch.to_radians().cos(),
        euler_angles.pitch.to_radians().sin(),
        euler_angles.yaw.to_radians().sin() * euler_angles.pitch.to_radians().cos()
    ))
}

pub fn s_update_angles(
    registry: &mut Registry,
    camera: &VersionedIndex,
    x_offset: f32,
    y_offset: f32,
    min_pitch: f32,
    max_pitch: f32
) -> Option<CEulerAngles> {
    let euler_angle = match registry.get_component_mut::<CEulerAngles>(camera) {
        Some(val) => val,
        _ => return None
    };
    euler_angle.pitch = (euler_angle.pitch + y_offset).clamp(min_pitch, max_pitch);
    euler_angle.yaw += x_offset;

    Some(CEulerAngles {
        pitch: euler_angle.pitch,
        yaw: euler_angle.yaw,
        ..CEulerAngles::default()
    })
}
