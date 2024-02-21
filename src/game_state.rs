use sdl2::event::Event;
use crate::{
    core,
    modules
};

pub struct FrameState {
    pub clear_color: glm::Vec4,
    pub editor_mode: bool,
    pub events: Vec<Event>,
    pub text_render: modules::text::TextRenderer,
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
