use crate::prelude::{
    qp_ecs::components::CSprite,
    qp_data::{
        IRenderer,
        FrameState
    },
    Registry
};

use super::super::batch_renderer::BatchRenderer;

pub struct RendererText {
    _camera: u64,
    _shader: u64,

    _renderer: BatchRenderer<10000, CSprite>
}

// TODO: properly integrate text drawing into pipeline
impl IRenderer for RendererText {
    fn draw(
        &mut self,
        _frame_state: &mut FrameState,
        _registry: &mut Registry
    ) -> Option<u32> {
        None
    }
}