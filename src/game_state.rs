use sdl2::event::Event;
use crate::modules;

pub struct FrameState {
    pub delta: f32,
    pub events: Vec<Event>,
    pub text_render: modules::text::TextRenderer,
    pub debug_mode: bool,
    pub debug_info: DebugInfo,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FrameResponse {
    Quit,
    None,
    Restart
}

#[derive(Debug, Default)]
pub struct DebugInfo {
    pub fps: u32,
    pub frame_ms: u32,

    pub editor_ms: u32,
    pub controller_ms: u32,
    pub render_ms: u32,
    pub draw_calls: u32,
}

pub fn set_frame_debug_info(app_state: &mut FrameState) {
    app_state.debug_info.fps = (1.0 / app_state.delta) as u32;
    app_state.debug_info.frame_ms = (app_state.delta * 1000.0) as u32;
}
