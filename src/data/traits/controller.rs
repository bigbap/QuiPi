use super::super::{
    structs::FrameState,
    enums::FrameResponse
};
use crate::prelude::Registry;

pub trait IController {
    fn update(
        &mut self,
        _frame_state: &mut FrameState,
        _registry: &mut Registry
    ) -> FrameResponse { FrameResponse::None }
}
