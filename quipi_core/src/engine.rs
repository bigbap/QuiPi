use std::fs;

use sdl2::event::Event;

use crate::{
    components::{
        register_components,
        CRGBA
    },
    resources::register_resources,
    systems::rendering::text::TextRenderer,
    utils::to_abs_path,
    QuiPiWindow,
    Registry
};

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
    pub clear_color: CRGBA,
    pub editor_mode: bool,
    pub events: Vec<Event>,
    pub text_render: TextRenderer,
    pub debug_info: DebugInfo,
    pub delta: f32,
}

#[derive(Debug, Default)]
pub struct DebugInfo {
    pub fps: f32,
    pub ms: f32
}

#[derive(Debug, Default)]
pub struct QPMouseState {
    pub pos: glm::Vec2,
    pub rel_pos: glm::Vec2,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FrameResponse {
    Quit,
    None,
    Restart
}

#[derive(Debug, PartialEq, Eq)]
pub enum AppMode {
    App,
    Editor
}

pub fn set_debug_info(app_state: &mut FrameState) {
    app_state.debug_info.fps = 1.0 / app_state.delta;
    app_state.debug_info.ms = app_state.delta * 1000.0;
}

pub fn create_asset_dirs() -> Result<(), Box<dyn std::error::Error>> {
    let asset_path = to_abs_path("assets")?;
    
    fs::create_dir_all(format!("{}/scenes", asset_path))?;
    fs::create_dir_all(format!("{}/shaders", asset_path))?;
    fs::create_dir_all(format!("{}/objects", asset_path))?;
    fs::create_dir_all(format!("{}/fonts", asset_path))?;

    Ok(())
}

pub fn setup() -> Result<Registry, Box<dyn std::error::Error>> {
    create_asset_dirs()?;

    let mut registry = Registry::init()?;

    register_components(&mut registry);
    register_resources(&mut registry);

    Ok(registry)
}
