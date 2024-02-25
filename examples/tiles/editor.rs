mod debug;

use crate::{
    QPError,
    qp_core::Timer,
    qp_data::{
        FrameResponse,
        FrameState,
        IController,
    },
    qp_editor::GuiManager,
    GlobalRegistry
};

pub struct AppEditor {
    gui: GuiManager,
    timer: Timer
}

impl AppEditor {
    pub fn new() -> Result<Self, QPError> {
        let mut gui = GuiManager::new(1.0)?;
        let debug_ui = debug::DebugUi {};

        gui.register_controller(debug_ui);

        Ok(Self {
            gui,
            timer: Timer::new()
        })
    }
}

impl IController for AppEditor {
    fn update(
        &mut self,
        frame_state: &mut FrameState,
        registry: &mut GlobalRegistry,
    ) -> FrameResponse {
        if !frame_state.debug_mode {
            return FrameResponse::None
        }

        self.timer.delta();

        self.gui.update(frame_state, registry);

        frame_state.debug_info.editor_ms = (self.timer.delta() * 1000.0) as u32;

        FrameResponse::None
    }
}