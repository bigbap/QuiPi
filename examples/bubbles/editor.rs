mod scene;
mod components;
mod debug;

use scene::SceneEditor;

use crate::{
    GUI,
    core::Timer,
    data::{
        FrameResponse,
        FrameState,
        IController,
    },
    Registry
};

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
}

impl IController for AppEditor {
    fn update(
        &mut self,
        frame_state: &mut FrameState,
        registry: &mut Registry,
    ) -> FrameResponse {
        if !frame_state.debug_mode {
            return FrameResponse::None
        }

        self.timer.delta();

        self.gui.begin_frame();

        self.scene.update(&self.gui, registry);
        debug::debug(&self.gui, frame_state, registry);

        self.gui.end_frame(frame_state).unwrap();

        frame_state.debug_info.editor_ms = (self.timer.delta() * 1000.0) as u32;

        FrameResponse::None
    }
}