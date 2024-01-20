use engine::math::random::Random;
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
    _camera: &VersionedIndex,
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
        _ => ()
    };

    Ok(Some(()))
}
