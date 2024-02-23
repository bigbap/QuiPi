use super::super::structs::FrameState;
use crate::prelude::Registry;

pub trait IRenderer {
    fn draw(
        &mut self,
        _frame_state: &mut FrameState,
        _registry: &mut Registry
    ) -> Option<u32> { None }
}