use crate::prelude::{
    Event,
    qp_gfx::TextRenderer
};

#[derive(Debug)]
pub struct FrameState {
    pub delta: f32,
    pub events: Vec<Event>,
    pub text_render: TextRenderer,
    pub debug_mode: bool,
    pub debug_info: DebugInfo,
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