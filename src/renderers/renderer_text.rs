use crate::{ecs::components::CSprite, rendering::batch::BatchRenderer, IRenderer};

pub struct RendererText {
    _camera: u64,
    _shader: u64,

    _renderer: BatchRenderer<10000, CSprite>
}

// TODO: properly integrate text drawing into pipeline
impl IRenderer for RendererText {
    fn draw(
        &mut self,
        _frame_state: &mut crate::FrameState,
        _registry: &mut crate::Registry
    ) -> Option<u32> {
        None
    }
}