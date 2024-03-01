use super::super::structs::FrameState;
use crate::prelude::{FrameResult, GlobalRegistry};

pub trait IController {
    fn update(
        &mut self,
        _frame_state: &mut FrameState,
        _registry: &mut GlobalRegistry,
    ) -> FrameResult;
}
