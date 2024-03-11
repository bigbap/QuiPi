pub mod input;
pub mod main_loop;
pub mod quad_shader;
pub mod render_base;
pub mod window;

use crate::plugin::*;

use self::{
    input::InputPlugin, main_loop::MainLoopPlugin, quad_shader::QuadShaderPlugin,
    render_base::RenderBasePlugin, window::WindowPlugin,
};

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

pub fn render_plugins() -> impl Plugins {
    (RenderBasePlugin {}, QuadShaderPlugin::default())
}
