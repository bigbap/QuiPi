pub mod input;
pub mod main_loop;
pub mod window;

use crate::plugin::*;

use self::{input::InputPlugin, main_loop::MainLoopPlugin, window::WindowPlugin};

pub fn default_plugins(title: &str, width: u32, height: u32) -> impl Plugins {
    (
        WindowPlugin {
            title: title.into(),
            width,
            height,
        },
        MainLoopPlugin {},
        InputPlugin {},
    )
}
