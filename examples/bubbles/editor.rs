mod components;
mod debug;
mod scene;

use quipi::app::Renderer;
use quipi::world::World;
use scene::SceneEditor;

use crate::editor::debug::DebugUi;
use crate::{qp_core::Timer, qp_editor::GuiManager, Controller, QPError};

pub struct AppEditor {
    gui: GuiManager,
    timer: Timer,
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
            timer: Timer::new(),
        })
    }
}

impl Renderer for AppEditor {
    fn draw(&mut self, world: &mut World) -> Option<u32> {
        if !world.debug_mode {
            return None;
        }

        self.timer.delta();

        self.gui.update(world);

        world.debug_info.editor_ms = (self.timer.delta() * 1000.0) as u32;

        None
    }
}
