pub mod renderer_2d;
pub mod renderer_text;

pub use renderer_2d::Renderer2D;
pub use renderer_text::RendererText;

use crate::{
    FrameState, Registry
};


pub trait IRenderer {
    fn draw(
        &mut self,
        _frame_state: &mut FrameState,
        _registry: &mut Registry
    ) -> Option<u32> { None }
}