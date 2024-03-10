use std::collections::HashMap;

use sdl2::{
    event::Event,
    keyboard::{Keycode, Mod},
};

use crate::resources::Resource;

#[derive(Resource)]
pub struct Input {
    key_state: HashMap<Keycode, Option<KeyState>>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            key_state: HashMap::<Keycode, Option<KeyState>>::new(),
        }
    }

    pub fn update_state(&mut self, event: &Event) {
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
            _ => (),
        }
    }

    pub fn peek(&self, keycode: Keycode) -> Option<KeyState> {
        match self.key_state.get(&keycode) {
            Some(state) => *state,
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState {
    pub timestamp: u32,
    pub keymod: Mod,
}
