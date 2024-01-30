use quipi::engine::FrameState;
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
    frame_state: &mut FrameState,
    registry: &mut Registry,
    rand: &mut Random
) -> Result<(), Box<dyn std::error::Error>> {
    for event in frame_state.event_pump.poll_iter() {
        match event {
            Event::Quit {..}|Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                frame_state.quit = true;
                return Ok(());
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

    Ok(())
}
