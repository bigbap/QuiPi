use engine::components::CEulerAngles;
use engine::math::random::Random;
use engine::systems::{
    rotation::s_rotate_camera,
    mvp_matrices::*
};
use engine::{
    Registry,
    VersionedIndex,
    gfx
};
use sdl2::event::{
    Event,
    WindowEvent
};
use sdl2::keyboard::Keycode;

use super::s_spawn_quad;

pub fn s_handle_input(
    camera: &VersionedIndex,
    registry: &mut Registry,
    event: sdl2::event::Event,
    rand: &mut Random
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
        Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
            s_spawn_quad(registry, rand)?;
        },
        Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
            if let Some(angles) = registry.get_component_mut::<CEulerAngles>(camera) {
                angles.pitch += 1.0;

                s_rotate_camera(registry, camera);
                s_set_ortho_projection_matrix(camera, registry);
                s_set_view_matrix(camera, registry);
            }
        },
        Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
            if let Some(angles) = registry.get_component_mut::<CEulerAngles>(camera) {
                angles.pitch -= 1.0;

                s_rotate_camera(registry, camera);
                s_set_ortho_projection_matrix(camera, registry);
                s_set_view_matrix(camera, registry);
            }
        },
        Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
            if let Some(angles) = registry.get_component_mut::<CEulerAngles>(camera) {
                angles.yaw -= 1.0;

                s_rotate_camera(registry, camera);
                s_set_ortho_projection_matrix(camera, registry);
                s_set_view_matrix(camera, registry);
            }
        },
        Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
            if let Some(angles) = registry.get_component_mut::<CEulerAngles>(camera) {
                angles.yaw += 1.0;

                s_rotate_camera(registry, camera);
                s_set_ortho_projection_matrix(camera, registry);
                s_set_view_matrix(camera, registry);
            }
        },
        _ => ()
    };

    Ok(Some(()))
}
