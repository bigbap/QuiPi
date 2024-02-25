use super::super::structs::FrameState;
use crate::prelude::GlobalRegistry;

pub trait IRenderer {
    fn draw(
        &mut self,
        _frame_state: &mut FrameState,
        _registry: &mut GlobalRegistry
    ) -> Option<u32> { None }
}