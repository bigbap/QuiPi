#![allow(dead_code)]

use egui::Vec2;

use crate::{
    components::{
        CScene,
        CTag
    },
    schemas::{
        entity2d::DEFAULT_RECT_TAG,
        ISchema,
        SchemaEntity2D
    },
    wrappers::egui::GUI,
    FrameState,
    Registry,
    VersionedIndex
};

use super::scene::save_scene_2d;

pub struct SceneEditor {
    gui: GUI,

    active_entity: Option<VersionedIndex>,
}

impl SceneEditor {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            gui: GUI::new(1.0)?,
            active_entity: None,
        })
    }

    pub fn update(
        &mut self,
        registry: &mut Registry,
        app_state: &mut FrameState
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.gui.begin_frame();
        self.entity_list(registry);
        self.debug(app_state, registry);
        self.gui.end_frame(app_state)
    }

    fn menu(&mut self) {
        self.gui.add_panel_top("Menu", |ui| {
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

    fn entity_list(
        &mut self,
        registry: &mut Registry
    ) {
        let entities = registry.entities.query::<CTag>(CTag { tag: DEFAULT_RECT_TAG.to_string() });

        self.gui.add_window("Scene", |ui| {
            ui.set_width(200.0);
            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("create entity").clicked() {
                    let schema = SchemaEntity2D::default();

                    if let Err(e) = schema.build(registry) {
                        println!("could not add entity: {}", e);
                    }
                }
                if ui.button("save scene").clicked() {
                    let scenes = registry.entities.query_all::<CScene>();
                    let Some(scene_id) = scenes.first() else { return };

                    if let Some(scene) = registry.entities.get::<CScene>(scene_id) {
                        if let Err(e) = save_scene_2d(&scene.name, *scene_id, &registry) {
                            println!("there was a problem saving scene {}: {:?}", scene.name, e);
                        }
                    }
                }
            });

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                for entity in entities.iter() {
                    ui.horizontal(|ui| {
                        ui.set_width(ui.available_width());
                        ui.radio_value(&mut self.active_entity, Some(*entity), entity.to_string());
                        if ui.button("del").clicked() {
                            registry.entities.set_to_delete(*entity);
                        }
                    });
                    ui.allocate_space(Vec2::new(0.0, 5.0));
                }
            });
        });
    }

    fn debug(&mut self, app_state: &FrameState, registry: &Registry) {
        self.gui.add_window("Debug Info", |ui| {
            ui.set_width(200.0);
            ui.label(format!("fps: {}", app_state.debug_info.fps));
            ui.label(format!("ms: {}", app_state.debug_info.ms));
            ui.separator();
            ui.label(format!("entity count: {}", registry.entities.count()));
            ui.label(format!("allocator size: {}", registry.entities.allocator_size()));
        })
    }
}

