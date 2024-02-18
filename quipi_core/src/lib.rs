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
pub mod schemas;

// pub use engine::run;
pub use core::ecs;
pub use core::ecs::Component;
pub use core::ecs::EntityManager;
pub use core::ecs::VersionedIndex;
pub use core::rendering;
pub use core::math;
pub use core::utils;
pub use registry::Registry;
pub use platform::sdl2::window::QuiPiWindow;
pub use platform::opengl;

use sdl2::event::Event;

pub trait QuiPiApp {
    /// game.init() is called by the engine, after all the Sdl and
    /// openGl setup is done.
    /// 
    /// Use this method to set up your game. If you do anything
    /// that uses the 'gl::' crate before this method gets called
    /// by the engine, you will get a 'function not loaded error'
    fn init(
        &mut self,
        registry: &mut Registry,
        winapi: &QuiPiWindow
    ) -> Result<(), Box<dyn std::error::Error>>;
    
    /// This method is called by the engine every frame.
    /// This is where you will do all your game specific logic.
    fn handle_frame(
        &mut self,
        registry: &mut Registry,
        frame_state: &mut FrameState
    ) -> Result<FrameResponse, Box<dyn std::error::Error>>;
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

