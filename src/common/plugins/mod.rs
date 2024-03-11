pub mod cameras;
pub mod clock;
pub mod core;
pub mod input;
pub mod main_loop;
pub mod quad_shader;
pub mod render_base;
pub mod schedules;
pub mod shaders;
pub mod textures;
pub mod window;

use crate::plugin::*;

use self::{
    core::CorePlugin, input::InputPlugin, main_loop::MainLoopPlugin, quad_shader::QuadShaderPlugin,
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
    )
}

pub fn render_plugins() -> impl Plugins {
    (RenderBasePlugin {}, QuadShaderPlugin::default())
}
