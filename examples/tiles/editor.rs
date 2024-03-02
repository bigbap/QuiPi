mod debug;

use quipi::{
    app::{Controller, FrameResult},
    world::World,
};

use crate::{qp_core::Timer, qp_editor::GuiManager, QPError};

pub struct AppEditor {
    gui: GuiManager,
    timer: Timer,
}

impl AppEditor {
    pub fn new() -> Result<Self, QPError> {
        let mut gui = GuiManager::new(1.0)?;
        let debug_ui = debug::DebugUi {};

        gui.register_controller(debug_ui);

        Ok(Self {
            gui,
            timer: Timer::new(),
        })
    }
}

impl Controller for AppEditor {
    fn update(&mut self, world: &mut World) -> FrameResult {
        if !world.debug_mode {
            return FrameResult::None;
        }

        self.timer.delta();

        self.gui.update(world);

        world.debug_info.editor_ms = (self.timer.delta() * 1000.0) as u32;

        FrameResult::None
    }
}
