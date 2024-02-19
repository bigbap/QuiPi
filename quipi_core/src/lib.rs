pub extern crate sdl2;
pub extern crate gl;
pub extern crate nalgebra_glm as glm;
pub extern crate freetype as ft;
pub extern crate serde;
// pub extern crate gltf;

pub mod core;
pub mod registry;
pub mod components;
pub mod resources;
pub mod systems;
pub mod platform;
pub mod schema;

// pub use engine::run;
pub use core::ecs;
pub use core::ecs::Component;
pub use core::ecs::EntityManager;
pub use core::ecs::VersionedIndex;
pub use core::rendering;
pub use core::math;
use core::rendering::RenderInfo;
pub use core::utils;
pub use registry::Registry;
pub use platform::sdl2::window::QuiPiWindow;
pub use platform::opengl;

use sdl2::event::Event;

pub trait IController {
    fn update(&mut self, _frame_state: &mut FrameState, _registry: &mut Registry) -> FrameResponse { FrameResponse::None }
    fn draw(&mut self, _frame_state: &mut FrameState,  _registry: &mut Registry) -> Option<RenderInfo> { None }
}

pub struct FrameState {
    pub clear_color: glm::Vec4,
    pub editor_mode: bool,
    pub events: Vec<Event>,
    pub text_render: core::text::TextRenderer,
    pub debug_info: DebugInfo,
    pub render_info: core::rendering::RenderInfo,
    pub editor_info: EditorInfo,
    pub delta: f32,
}

#[derive(Debug, Default)]
pub struct EditorInfo {
    pub ms: u32
}

#[derive(Debug, PartialEq, Eq)]
pub enum FrameResponse {
    Quit,
    None,
    Restart
}

#[derive(Debug, Default)]
pub struct QPMouseState {
    pub pos: glm::Vec2,
    pub rel_pos: glm::Vec2,
}

#[derive(Debug, Default)]
pub struct DebugInfo {
    pub fps: u32,
    pub ms: u32
}

#[derive(Debug, PartialEq, Eq)]
pub enum AppMode {
    App,
    Editor
}

pub fn set_debug_info(app_state: &mut FrameState) {
    app_state.debug_info.fps = (1.0 / app_state.delta) as u32;
    app_state.debug_info.ms = (app_state.delta * 1000.0) as u32;
}

