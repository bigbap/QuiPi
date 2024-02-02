use egui::{Modifiers, Pos2, pos2, Key};
use sdl2::{
    event::Event::{ self, * },
    keyboard::{
        Keycode::{ self, * },
        Mod
    },
    mouse::MouseButton,
};

use super::painter::Painter;

pub fn parse_event(
    event: &Event,
    painter: &Painter
) -> Option<egui::Event> {
    match event {
        MouseButtonDown { mouse_btn, x, y, .. } => parse_mouse_btn(
            *mouse_btn,
            true,
            *x as f32,
            *y as f32
        ),
        MouseButtonUp { mouse_btn, x, y, .. } => parse_mouse_btn(
            *mouse_btn,
            false,
            *x as f32,
            *y as f32
        ),
        MouseMotion { x, y, .. } => Some(egui::Event::PointerMoved(pos2(
            *x as f32 / painter.pixels_per_point,
            *y as f32 / painter.pixels_per_point
        ))),
        KeyDown { keycode, keymod, repeat, .. } => parse_key(*keycode, *keymod, *repeat, true),
        KeyUp { keycode, keymod, repeat, .. } => parse_key(*keycode, *keymod, *repeat, false),
        TextInput { text, .. } => Some(egui::Event::Text(text.to_string())),
        _ => None
    }
}

fn parse_key(
    keycode: Option<Keycode>,
    keymod: Mod,
    repeat: bool,
    state: bool // true = keydown, false = keyup
) -> Option<egui::Event> {
    let key_code = match keycode {
        Some(key_code) => key_code,
        _ => return None
    };

    let key = match key_code {
        A => Key::A,
        B => Key::B,
        C => Key::C,
        D => Key::D,
        E => Key::E,
        F => Key::F,
        G => Key::G,
        H => Key::H,
        I => Key::I,
        J => Key::J,
        K => Key::K,
        L => Key::L,
        M => Key::M,
        N => Key::N,
        O => Key::O,
        P => Key::P,
        Q => Key::Q,
        R => Key::R,
        S => Key::S,
        T => Key::T,
        U => Key::U,
        V => Key::V,
        W => Key::W,
        X => Key::X,
        Y => Key::Y,
        Z => Key::Z,

        Kp0 | Num0 => Key::Num0,
        Kp1 | Num1 => Key::Num1,
        Kp2 | Num2 => Key::Num2,
        Kp3 | Num3 => Key::Num3,
        Kp4 | Num4 => Key::Num4,
        Kp5 | Num5 => Key::Num5,
        Kp6 | Num6 => Key::Num6,
        Kp7 | Num7 => Key::Num7,
        Kp8 | Num8 => Key::Num8,
        Kp9 | Num9 => Key::Num7,

        Return => Key::Enter,
        Space => Key::Space,
        Tab => Key::Tab,
        Backspace => Key::Backspace,
        Insert => Key::Insert,
        Delete => Key::Delete,
        Home => Key::Home,
        End => Key::End,
        Left => Key::ArrowLeft,
        Right => Key::ArrowRight,
        Up => Key::ArrowUp,
        Down => Key::ArrowDown,
        PageUp => Key::PageUp,
        PageDown => Key::PageDown,

        _ => return None
    };

    let modifiers = Modifiers {
        alt: (keymod & Mod::LALTMOD == Mod::LALTMOD) || (keymod & Mod::RALTMOD == Mod::RALTMOD),
        ctrl: (keymod & Mod::LCTRLMOD == Mod::LCTRLMOD) || (keymod & Mod::RCTRLMOD == Mod::RCTRLMOD),
        shift: (keymod & Mod::LSHIFTMOD == Mod::LSHIFTMOD) || (keymod & Mod::RSHIFTMOD == Mod::RSHIFTMOD),
        command: keymod & Mod::LCTRLMOD == Mod::LCTRLMOD,
        mac_cmd: false,
    };

    Some(egui::Event::Key {
        key,
        physical_key: None,
        pressed: state,
        repeat,
        modifiers,
    })
}

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
