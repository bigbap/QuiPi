use egui::{Modifiers, Pos2};
use sdl2::{
    event::Event::{ self, * },
    keyboard::{Keycode, Mod},
    mouse::MouseButton,
};

pub fn parse_event(
    event: &Event
) -> Option<egui::Event> {
    match event {
        MouseButtonDown { mouse_btn, x, y, .. } => return parse_mouse_btn(
            *mouse_btn,
            true,
            *x as f32,
            *y as f32
        ),
        MouseButtonUp { mouse_btn, x, y, .. } => return parse_mouse_btn(
            *mouse_btn,
            false,
            *x as f32,
            *y as f32
        ),
        KeyDown { keycode, keymod, repeat, .. } =>
            parse_key(*keycode, *keymod, *repeat),
        KeyUp { keycode, keymod, repeat, .. } =>
            parse_key(*keycode, *keymod, *repeat),
        _ => ()
    }

    None
}

fn parse_key(
    keycode: Option<Keycode>,
    keymod: Mod,
    repeat: bool
) {}

fn parse_mouse_btn(
    btn: MouseButton,
    state: bool, // true = pressed, false = released
    x: f32,
    y: f32
) -> Option<egui::Event> {
    let btn = match btn {
        MouseButton::Left => Some(egui::PointerButton::Primary),
        _ => None
    };

    if let Some(btn) = btn {
        return Some(egui::Event::PointerButton {
            pos: Pos2::new(x, y),
            button: btn,
            pressed: state,
            modifiers: Modifiers::NONE
        })
    }

    None
}
