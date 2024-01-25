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
use sdl2::mouse::MouseButton;

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
        Event::KeyDown { keycode, repeat: false, .. } => {
            if let Some(vel) = registry.get_component_mut::<CVelocity>(camera) {
                match keycode {
                    Some(Keycode::W) => vel.z = 5.0,
                    Some(Keycode::S) => vel.z = -5.0,
                    Some(Keycode::A) => vel.x = -5.0,
                    Some(Keycode::D) => vel.x = 5.0,
                    _ => ()
                }
            }
        },
        Event::KeyUp { keycode, repeat: false, .. } => {
            if let Some(vel) = registry.get_component_mut::<CVelocity>(camera) {
                match keycode {
                    Some(Keycode::W) | Some(Keycode::S) => vel.z = 0.0,
                    Some(Keycode::A) | Some(Keycode::D) => vel.x = 0.0,
                    _ => ()
                }
            }
        },
        Event::MouseButtonDown { mouse_btn: MouseButton::Middle, .. } => {
            if let Some(mouse_btn_state) = registry.get_component_mut::<CMouseBtnState>(camera) {
                mouse_btn_state.btn_middle = true;
            }
        },
        Event::MouseButtonUp { mouse_btn: MouseButton::Middle, .. } => {
            if let Some(mouse_btn_state) = registry.get_component_mut::<CMouseBtnState>(camera) {
                mouse_btn_state.btn_middle = false;
            }
        },
        Event::MouseMotion { xrel, yrel, .. } => {
            let mut mov = (0.0, 0.0);
            if let Some(mouse_btn_state) = registry.get_component::<CMouseBtnState>(camera) {
                if mouse_btn_state.btn_middle {
                    mov = (xrel as f32, yrel as f32);
                }
            }
            if let Some(angles) = registry.get_component_mut::<CEulerAngles>(camera) {
                angles.pitch += mov.1 * 0.5;
                angles.yaw += mov.0 * 0.5;
            }
        }
        _ => ()
    };

    Ok(Some(()))
}

