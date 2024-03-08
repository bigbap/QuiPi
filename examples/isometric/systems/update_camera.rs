use quipi::{
    components::CVelocity,
    systems::{
        movement::s_apply_velocity, mvp_matrices::s_set_view_matrix, rotation::s_rotate_camera,
    },
    GlobalRegistry, Index,
};

pub fn s_update_camera(
    camera: &Index,
    registry: &mut GlobalRegistry,
    delta: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(vel) = registry.get_component::<CVelocity>(camera) {
        s_apply_velocity(registry, camera, delta, glm::vec3(vel.x, vel.y, vel.z))?;
    }

    s_rotate_camera(registry, camera);
    s_set_view_matrix(camera, registry);

    Ok(())
}
