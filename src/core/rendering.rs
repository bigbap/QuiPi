pub mod batch;
pub mod texture;
pub mod vertex;

#[derive(Debug, Default, Clone)]
pub struct RenderInfo {
    pub num_draw_calls: u32,
    pub total_ms: f32
}