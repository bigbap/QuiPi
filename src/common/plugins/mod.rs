pub mod cameras;
pub mod clock;
pub mod core;
pub mod input;
pub mod main_loop;
pub mod render_2d;
pub mod render_base;
pub mod schedules;
pub mod shaders;
pub mod textures;
pub mod window;

use crate::plugin::*;

use self::{
    core::CorePlugin, input::InputPlugin, main_loop::MainLoopPlugin, render_2d::Render2DPlugin,
    render_base::RenderBasePlugin, schedules::SchedulesPlugin, window::WindowPlugin,
};

pub fn mandatory_plugins() -> impl Plugins {
    (CorePlugin {}, SchedulesPlugin {})
}

pub fn window_plugins(title: &str, width: u32, height: u32) -> impl Plugins {
    (
        WindowPlugin {
            title: title.into(),
            width,
            height,
        },
        MainLoopPlugin {},
        InputPlugin {},
        RenderBasePlugin {},
    )
}

pub fn plugins_2d(title: &str, width: u32, height: u32) -> impl Plugins {
    (
        window_plugins(title, width, height),
        Render2DPlugin::default(),
    )
}
