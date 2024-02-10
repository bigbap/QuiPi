#![allow(dead_code)]

use egui::Vec2;

use crate::{
    components::{
        CScene,
        CTag, CTransform, CVelocity, CRGBA
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
        self.entity_components(registry);
        self.debug(app_state, registry);
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

    fn entity_list(
        &mut self,
        registry: &mut Registry
    ) {
        let entities = registry.entities.query::<CTag>(CTag { tag: DEFAULT_RECT_TAG.to_string() });

        egui::Window::new("Scene").show(&self.gui.ctx, |ui| {
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
                    // let name = registry.entities.get::<CName>(entity).unwrap();
                    // let name = name.name.clone();

                    ui.horizontal(|ui| {
                        ui.set_width(ui.available_width());
                        ui.radio_value(&mut self.active_entity, Some(*entity), entity.to_string());
                        if ui.button("del").clicked() {
                            if self.active_entity == Some(*entity) {
                                self.active_entity = None;
                            }
                            registry.entities.set_to_delete(*entity);
                        }
                    });
                    ui.allocate_space(Vec2::new(0.0, 5.0));
                }
            });
        });
    }

    fn entity_components(&mut self, registry: &mut Registry) {
        if let Some(entity) = self.active_entity {
            egui::Window::new("Entity").show(&self.gui.ctx, |ui| {
                if let Some(transform) = registry.entities.get_mut::<CTransform>(&entity) {
                    ui.strong("Transforms");
                    ui.label("translate");
                    ui.horizontal(|ui| {
                        ui.label("x");
                        ui.add(egui::DragValue::new(&mut transform.translate.x).speed(1.0));
                        ui.label("y");
                        ui.add(egui::DragValue::new(&mut transform.translate.y).speed(1.0));
                        ui.label("z");
                        ui.add(egui::DragValue::new(&mut transform.translate.z).speed(1.0));
                    });
                    ui.label("scale");
                    ui.horizontal(|ui| {
                        ui.label("x");
                        ui.add(egui::DragValue::new(&mut transform.scale.x).speed(0.05));
                        ui.label("y");
                        ui.add(egui::DragValue::new(&mut transform.scale.y).speed(0.05));
                        ui.label("z");
                        ui.add(egui::DragValue::new(&mut transform.scale.z).speed(0.05));
                    });
                    ui.label("rotation");
                    ui.horizontal(|ui| {
                        ui.label("x");
                        ui.add(egui::DragValue::new(&mut transform.rotate.x).speed(0.1));
                        ui.label("y");
                        ui.add(egui::DragValue::new(&mut transform.rotate.y).speed(0.1));
                        ui.label("z");
                        ui.add(egui::DragValue::new(&mut transform.rotate.z).speed(0.1));
                        ui.label("angle");
                        ui.add(egui::DragValue::new(&mut transform.angle).speed(0.1));
                    });
                }
                if let Some(velocity) = registry.entities.get_mut::<CVelocity>(&entity) {
                    ui.strong("Velocity");
                    ui.horizontal(|ui| {
                        ui.label("x");
                        ui.add(egui::DragValue::new(&mut velocity.x).speed(1.0));
                        ui.label("y");
                        ui.add(egui::DragValue::new(&mut velocity.y).speed(1.0));
                        ui.label("z");
                        ui.add(egui::DragValue::new(&mut velocity.z).speed(1.0));
                    });
                }
                if let Some(color) = registry.entities.get_mut::<CRGBA>(&entity) {
                    ui.strong("Color");
                    ui.color_edit_button_rgba_premultiplied(&mut [color.r, color.g, color.b, color.a]);
                }
            });
        }
    }

    fn debug(&mut self, app_state: &FrameState, registry: &Registry) {
        egui::Window::new("Debug Info")
            .show(&self.gui.ctx, |ui| {
                ui.set_width(200.0);
                ui.label(format!("fps: {}", app_state.debug_info.fps));
                ui.label(format!("ms: {}", app_state.debug_info.ms));
                ui.separator();
                ui.label(format!("entity count: {}", registry.entities.count()));
                ui.label(format!("allocator size: {}", registry.entities.allocator_size()));
            });
    }
}

