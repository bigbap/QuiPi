use crate::{FrameResponse, FrameState, Registry};

pub trait IController {
    fn update(
        &mut self,
        _frame_state: &mut FrameState,
        _registry: &mut Registry
    ) -> FrameResponse { FrameResponse::None }
}