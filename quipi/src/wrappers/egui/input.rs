use sdl2::{
    event::Event,
    keyboard::{Keycode, Mod},
};

pub fn parse_event(
    event: &Event
) -> Option<egui::Event> {
    let mut parsed = None;

    match event {
        Event::KeyDown { keycode, keymod, repeat, .. } =>
            parse_key(*keycode, *keymod, *repeat),
        Event::KeyUp { keycode, keymod, repeat, .. } =>
            parse_key(*keycode, *keymod, *repeat),
        _ => ()
    }

    parsed
}

fn parse_key(
    keycode: Option<Keycode>,
    keymod: Mod,
    repeat: bool
) {}
