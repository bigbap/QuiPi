use quipi::{
    VersionedIndex,
    GlobalRegistry,
    systems::{
        rotation::s_rotate_camera,
        mvp_matrices::s_set_view_matrix,
        movement::s_apply_velocity
    },
    components::CVelocity
};

pub fn s_update_camera(
    camera: &VersionedIndex,
    registry: &mut GlobalRegistry,
    delta: f32
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(vel) = registry.get_component::<CVelocity>(camera) {
        s_apply_velocity(registry, camera, delta, glm::vec3(vel.x, vel.y, vel.z))?;
    }

    s_rotate_camera(registry, camera);
    s_set_view_matrix(camera, registry);

    Ok(())
}
