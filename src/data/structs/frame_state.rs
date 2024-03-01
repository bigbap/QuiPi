use crate::prelude::{qp_gfx::QPText, Event};

#[derive(Debug)]
pub struct World {
    pub delta: f32,
    pub debug_mode: bool,
    pub debug_info: DebugInfo,
    pub viewport: Viewport,
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

#[derive(Debug)]
pub struct Viewport {
    pub width: i32,
    pub height: i32,
}
