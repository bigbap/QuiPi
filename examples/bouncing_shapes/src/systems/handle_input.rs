use engine::math::random::Random;
use engine::{
    Registry,
    VersionedIndex
};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use super::s_spawn_quad;

pub fn s_handle_input(
    _camera: &VersionedIndex,
    registry: &mut Registry,
    event: sdl2::event::Event,
    rand: &mut Random
) -> Result<Option<()>, Box<dyn std::error::Error>> {
    match event {
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Ok(None),
        Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
            s_spawn_quad(registry, (0.2, 0.3, 0.4), rand)?;
        },
        _ => ()
    };

    Ok(Some(()))
}
