use quipi::{
    FrameResponse,
    Registry,
};
use quipi_core::core::canvas;
use quipi_core::FrameState;
use sdl2::event::{
    Event,
    WindowEvent
};
use sdl2::keyboard::Keycode;

use super::spawner::RectSpawner;

pub fn handle_input(
    frame_state: &mut FrameState,
    registry: &mut Registry,
    spawner: &mut RectSpawner
) -> Result<FrameResponse, Box<dyn std::error::Error>> {
    for event in frame_state.events.iter() {
        match event {
            Event::Quit {..} => {
                return Ok(FrameResponse::Quit);
            },
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                if cfg!(debug_assertions) {
                    frame_state.editor_mode = !frame_state.editor_mode;
                }
            },
            Event::KeyDown { keycode, .. } => {
                if frame_state.editor_mode { continue; }
                match keycode {
                    Some(Keycode::Space) => {
                        spawner.spawn(registry)?;
                    },
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
