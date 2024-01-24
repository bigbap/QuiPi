use engine::systems::movement::s_apply_velocity;
use engine::{
    Registry,
    VersionedIndex,
    components::*
};
use engine::{
    gfx,
    systems::{
        mvp_matrices::*,
        rotation::*
    }
};
use sdl2::event::{
    Event,
    WindowEvent
};
use sdl2::keyboard::Keycode;

pub fn s_handle_input(
    registry: &mut Registry,
    camera: &VersionedIndex,
    event: sdl2::event::Event,
) -> Result<Option<()>, Box<dyn std::error::Error>> {
    match event {
        Event::Quit {..} => return Ok(None),
        Event::Window {
            win_event: WindowEvent::Resized(w, h),
            ..
        } => {
            gfx::view::adjust_viewport_dims(w, h);
        },
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Ok(None),
        Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
            if let Some(angles) = registry.get_component_mut::<CEulerAngles>(camera) {
                angles.pitch += 1.0;

                update_camera(registry, camera);
            }
        },
        Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
            if let Some(angles) = registry.get_component_mut::<CEulerAngles>(camera) {
                angles.pitch -= 1.0;

                update_camera(registry, camera);
            }
        },
        Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
            if let Some(angles) = registry.get_component_mut::<CEulerAngles>(camera) {
                angles.yaw -= 1.0;

                update_camera(registry, camera);
            }
        },
        Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
            if let Some(angles) = registry.get_component_mut::<CEulerAngles>(camera) {
                angles.yaw += 1.0;

                update_camera(registry, camera);
            }
        },
        Event::KeyDown { keycode: Some(Keycode::W), .. } => {
            let velocity = glm::vec3(0.0, 0.0, 0.5);

            s_apply_velocity(registry, camera, 0.3, velocity)?;
            update_camera(registry, camera);
        },
        Event::KeyDown { keycode: Some(Keycode::S), .. } => {
            let velocity = glm::vec3(0.0, 0.0, -0.5);

            s_apply_velocity(registry, camera, 0.3, velocity)?;
            update_camera(registry, camera);
        },
        Event::KeyDown { keycode: Some(Keycode::A), .. } => {
            let velocity = glm::vec3(0.0, -0.5, 0.0);

            s_apply_velocity(registry, camera, 0.3, velocity)?;
            update_camera(registry, camera);
        },
        Event::KeyDown { keycode: Some(Keycode::D), .. } => {
            let velocity = glm::vec3(0.0, 0.5, 0.0);

            s_apply_velocity(registry, camera, 0.3, velocity)?;
            update_camera(registry, camera);
        },
        Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
            let velocity = glm::vec3(0.5, 0.0, 0.0);

            s_apply_velocity(registry, camera, 0.3, velocity)?;
            update_camera(registry, camera);
        },
        Event::KeyDown { keycode: Some(Keycode::E), .. } => {
            let velocity = glm::vec3(-0.5, 0.0, 0.0);

            s_apply_velocity(registry, camera, 0.3, velocity)?;
            update_camera(registry, camera);
        },
        _ => ()
    };

    Ok(Some(()))
}

fn update_camera(
    registry: &mut Registry,
    camera: &VersionedIndex
) {
    s_rotate_camera(registry, camera);
    s_set_projection_matrix(camera, registry);
    s_set_view_matrix(camera, registry);
}
