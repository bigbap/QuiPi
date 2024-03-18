use std::collections::HashMap;

use sdl2::{
    event::Event,
    keyboard::{Keycode, Mod},
};

use crate::resources::*;

#[derive(Resource, AsAny)]
pub struct Input {
    key_state: HashMap<Keycode, Option<KeyState>>,
    mouse_state: MouseState,
}

impl Input {
    pub fn new() -> Self {
        Self {
            key_state: HashMap::<Keycode, Option<KeyState>>::new(),
            mouse_state: MouseState::default(),
        }
    }

    pub fn update_state(&mut self, event: &Event) {
        self.mouse_state.dirty = false;

        match event {
            Event::KeyDown {
                timestamp,
                keycode: Some(keycode),
                keymod,
                repeat: false,
                ..
            } => {
                self.key_state.insert(
                    *keycode,
                    Some(KeyState {
                        timestamp: *timestamp,
                        keymod: *keymod,
                    }),
                );
            }
            Event::KeyUp {
                keycode: Some(keycode),
                repeat: false,
                ..
            } => {
                self.key_state.insert(*keycode, None);
            }
            Event::MouseMotion {
                x, y, xrel, yrel, ..
            } => {
                self.mouse_state.position = MousePosition {
                    x: *x,
                    y: *y,
                    xrel: *xrel,
                    yrel: *yrel,
                };
                self.mouse_state.dirty = true
            }
            _ => (),
        }
    }

    pub fn peek(&self, keycode: Keycode) -> Option<KeyState> {
        match self.key_state.get(&keycode) {
            Some(state) => *state,
            _ => None,
        }
    }

    pub fn mouse_moved(&self) -> bool {
        self.mouse_state.dirty
    }

    pub fn mouse_position(&self) -> &MousePosition {
        &self.mouse_state.position
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState {
    pub timestamp: u32,
    pub keymod: Mod,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MouseState {
    pub dirty: bool,
    pub position: MousePosition,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MousePosition {
    pub x: i32,
    pub y: i32,
    pub xrel: i32,
    pub yrel: i32,
}
