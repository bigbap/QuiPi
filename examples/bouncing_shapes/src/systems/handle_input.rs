use quipi::FrameResponse;
use quipi::engine::AppState;
use quipi::math::random::Random;
use quipi::{
    Registry,
    systems::rendering::canvas
};
use sdl2::event::{
    Event,
    WindowEvent
};
use sdl2::keyboard::Keycode;

use super::s_spawn_quad;

pub fn s_handle_input(
    app_state: &mut AppState,
    registry: &mut Registry,
    rand: &mut Random
) -> Result<FrameResponse, Box<dyn std::error::Error>> {
    for event in app_state.events.iter() {
        match event {
            Event::Quit {..} => {
                return Ok(FrameResponse::Quit);
            },
            Event::KeyDown { keycode, .. } => {
                if app_state.editor_mode { continue; }
                match keycode {
                    Some(Keycode::F11) => {
                        if cfg!(debug_assertions) {
                            app_state.editor_mode = true;
                        }
                    },
                    Some(Keycode::Space) => { s_spawn_quad(registry, rand)?; },
                    Some(Keycode::Escape) => return Ok(FrameResponse::Quit),
                    _ => ()
                }
            },
            Event::Window {
                win_event: WindowEvent::Resized(w, h),
                ..
            } => {
                canvas::set_dimensions(0, 0, *w, *h);
            },
            _ => ()
        };
    }

    Ok(FrameResponse::Ignore)
}
