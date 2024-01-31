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
    for event in app_state.winapi.get_event_queue()?.poll_iter() {
        match event {
            Event::Quit {..} => {
                return Ok(FrameResponse::Quit);
            },
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                #[cfg(debug_assertions)]
                return Ok(FrameResponse::RelinquishInput);
            },
            Event::Window {
                win_event: WindowEvent::Resized(w, h),
                ..
            } => {
                canvas::set_dimensions(0, 0, w, h);
            },
            Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                s_spawn_quad(registry, rand)?;
            },
            _ => ()
        };
    }

    Ok(FrameResponse::Ignore)
}
