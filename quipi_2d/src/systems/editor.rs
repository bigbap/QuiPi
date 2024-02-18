#![allow(dead_code)]

use quipi_core::{EditorInfo, utils::Timer, platform::egui::GUI};

use crate::{
    FrameState,
    Registry
};

use self::scene::SceneEditor;

mod scene;
mod components;
mod debug;

pub struct AppEditor {
    gui: GUI,
    scene: SceneEditor,
    timer: Timer
}

impl AppEditor {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            gui: GUI::new(1.0)?,
            scene: SceneEditor::new(),
            timer: Timer::new()
        })
    }

    pub fn update(
        &mut self,
        registry: &mut Registry,
        app_state: &mut FrameState,
    ) -> Result<EditorInfo, Box<dyn std::error::Error>> {
        self.timer.delta();

        self.gui.begin_frame();

        self.scene.update(&self.gui, registry);
        debug::debug(&self.gui, app_state, registry);

        self.gui.end_frame(app_state)?;

        Ok(EditorInfo {
            ms: (self.timer.delta() * 1000.0) as u32
        })
    }
}

