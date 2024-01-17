use engine::components::CVelocity;
use engine::{
    Registry,
    VersionedIndex
};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const CAMERA_SPEED: f32 = 2.0;

pub fn s_handle_input(
    camera: &VersionedIndex,
    registry: &mut Registry,
    event: sdl2::event::Event
) -> Result<Option<()>, Box<dyn std::error::Error>> {
    let velocity = registry.get_component_mut::<CVelocity>(camera).unwrap();

    match event {
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Ok(None),
        Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } => velocity.y -= CAMERA_SPEED,
        Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => velocity.y += CAMERA_SPEED,
        Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => velocity.x += CAMERA_SPEED,
        Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => velocity.x -= CAMERA_SPEED,
        Event::KeyUp { keycode: Some(Keycode::W), repeat: false, .. } => velocity.y += CAMERA_SPEED,
        Event::KeyUp { keycode: Some(Keycode::S), repeat: false, .. } => velocity.y -= CAMERA_SPEED,
        Event::KeyUp { keycode: Some(Keycode::A), repeat: false, .. } => velocity.x -= CAMERA_SPEED,
        Event::KeyUp { keycode: Some(Keycode::D), repeat: false, .. } => velocity.x += CAMERA_SPEED,
        // Event::MouseMotion { xrel, yrel, .. } => {
        //     let sensitivity = 0.1;
        //     let euler_angles = s_update_angles(
        //         &mut self.registry,
        //         &self.camera,
        //         xrel as f32 * sensitivity,
        //         yrel as f32 * sensitivity,
        //         -89.0,
        //         89.0
        //     ).unwrap();
        //
        //     s_rotate(
        //         &mut self.registry,
        //         &self.camera,
        //         euler_angles
        //     );
        // },
        _ => ()
    };

    Ok(Some(()))
}
