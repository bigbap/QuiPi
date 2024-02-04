use quipi::FrameResponse;
use quipi::engine::AppState;
use quipi::{
    Registry,
    systems::rendering::canvas
};
use sdl2::event::{
    Event,
    WindowEvent
};
use sdl2::keyboard::Keycode;

use super::spawner::RectSpawner;

pub fn s_handle_input(
    app_state: &mut AppState,
    registry: &mut Registry,
    spawner: &mut RectSpawner
) -> Result<FrameResponse, Box<dyn std::error::Error>> {
    for event in app_state.events.iter() {
        match event {
            Event::Quit {..} => {
                return Ok(FrameResponse::Quit);
            },
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                if cfg!(debug_assertions) {
                    app_state.editor_mode = !app_state.editor_mode;
                }
            },
            Event::KeyDown { keycode, .. } => {
                if app_state.editor_mode { continue; }
                match keycode {
                    Some(Keycode::Space) => { spawner.spawn(registry)?; },
                    Some(Keycode::W) => (), // placeholder
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

    Ok(FrameResponse::None)
}
