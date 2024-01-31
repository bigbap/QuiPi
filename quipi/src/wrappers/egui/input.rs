use sdl2::{event::Event, keyboard::Keycode};

use crate::wrappers::sdl2::window::QuiPiWindow;

pub fn parse_input(
    window_api: &mut QuiPiWindow
) -> egui::RawInput {
    for event in window_api.get_event_queue().unwrap().poll_iter() {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => (),
            Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => (),
            Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => (),
            _ => ()
        }
    }
    egui::RawInput::default()
}
