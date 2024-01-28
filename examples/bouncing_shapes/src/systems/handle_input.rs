use skald::gfx::canvas::Canvas;
use skald::math::random::Random;
use skald::{
    Registry,
    gfx
};
use sdl2::event::{
    Event,
    WindowEvent
};
use sdl2::keyboard::Keycode;

use super::s_spawn_quad;

pub fn s_handle_input(
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
            gfx::canvas::set_dimensions(&Canvas { x: 0, y: 0, width: w, height: h });
        },
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Ok(None),
        Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
            s_spawn_quad(registry, rand)?;
        },
        _ => ()
    };

    Ok(Some(()))
}
