pub mod input;
pub mod main;
pub mod renderer_2d;
pub mod window;

pub use crate::plugin::*;

use self::{input::InputPlugin, main::MainLoopPlugin, window::WindowPlugin};

pub fn default_plugins() -> impl Plugins {
    (WindowPlugin {}, MainLoopPlugin {}, InputPlugin {})
}

pub fn plugins_2d() -> impl Plugins {}
