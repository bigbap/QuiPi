#![allow(dead_code)]

use quipi_core::wrappers::egui::GUI;

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
    scene: SceneEditor
}

impl AppEditor {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            gui: GUI::new(1.0)?,
            scene: SceneEditor::new()
        })
    }

    pub fn update(
        &mut self,
        registry: &mut Registry,
        app_state: &mut FrameState
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.gui.begin_frame();

        self.scene.update(&self.gui, registry);
        debug::debug(&self.gui, app_state, registry);

        self.gui.end_frame(app_state)
    }

    fn menu(&mut self) {
        egui::TopBottomPanel::top("Menu").show(&self.gui.ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save").clicked() {
                        println!("saved from editor");
                    }
                    if ui.button("Quit").clicked() {
                        println!("Quiting from editor");
                    }
                });
            });
        });
    }
}

