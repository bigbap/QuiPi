use super::super::{
    structs::FrameState,
    enums::FrameResponse
};
use crate::prelude::GlobalRegistry;

pub trait IController {
    fn update(
        &mut self,
        _frame_state: &mut FrameState,
        _registry: &mut GlobalRegistry
    ) -> FrameResponse { FrameResponse::None }
}
