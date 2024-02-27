mod scene;
mod components;
mod debug;

use quipi::prelude::qp_data::IRenderer;
use scene::SceneEditor;

use crate::{
    QPError,
    qp_editor::GuiManager,
    qp_core::Timer,
    qp_data::{
        FrameState,
        IController,
    },
    GlobalRegistry
};
use crate::editor::debug::DebugUi;

pub struct AppEditor {
    gui: GuiManager,
    timer: Timer
}

impl AppEditor {
    pub fn new() -> Result<Self, QPError> {
        let mut gui = GuiManager::new(1.0)?;
        let scene = SceneEditor::new();
        let debug_ui = DebugUi {};

        gui.register_controller(scene);
        gui.register_controller(debug_ui);

        Ok(Self {
            gui,
            timer: Timer::new()
        })
    }
}

impl IRenderer for AppEditor {
    fn draw(
        &mut self,
        frame_state: &mut FrameState,
        registry: &mut GlobalRegistry,
    ) -> Option<u32> {
        if !frame_state.debug_mode {
            return None
        }

        self.timer.delta();

        self.gui.update(frame_state, registry);

        frame_state.debug_info.editor_ms = (self.timer.delta() * 1000.0) as u32;

        None
    }
}