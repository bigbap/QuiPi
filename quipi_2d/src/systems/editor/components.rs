use egui::Ui;

use crate::{
    components::{
        CQuad,
        CTag,
        CTransform2D,
        CVelocity2D,
    },
    Registry,
    VersionedIndex,
    GUI
};

pub struct EntityEditor {
    pub active_entity: Option<VersionedIndex>,

    to_remove: Vec<Box<dyn FnMut(&mut Registry, VersionedIndex)>>
}

impl EntityEditor {
    pub fn new() -> Self {
        Self {
            active_entity: None,
            to_remove: vec![]
        }
    }

    pub fn update(
        &mut self,
        gui: &GUI,
        registry: &mut Registry
    ) {
        if let Some(entity) = self.active_entity {
            while !self.to_remove.is_empty() {
                self.to_remove.pop().unwrap()(registry, entity);
            }

            egui::Window::new("Entity").show(&gui.ctx, |ui| {
                ui.add_space(10.0);
                self.add_component(ui, entity, registry);
                ui.add_space(10.0);

                if let Some(tag) = registry.entities.get_mut::<CTag>(&entity) {
                    ui.collapsing("Tag", |ui| {
                        ui.add(egui::TextEdit::singleline(&mut tag.tag));
                    });
                }
                if let Some(transform) = registry.entities.get_mut::<CTransform2D>(&entity) {
                    ui.collapsing("Transforms", |ui| {
                        ui.label("translate");
                        ui.horizontal(|ui| {
                            ui.label("x");
                            ui.add(egui::DragValue::new(&mut transform.translate.x).speed(1.0));
                            ui.label("y");
                            ui.add(egui::DragValue::new(&mut transform.translate.y).speed(1.0));
                        });
                        ui.label("scale");
                        ui.horizontal(|ui| {
                            ui.label("x");
                            ui.add(egui::DragValue::new(&mut transform.scale.x).speed(0.05));
                            ui.label("y");
                            ui.add(egui::DragValue::new(&mut transform.scale.y).speed(0.05));
                        });
                        ui.label("rotation");
                        ui.horizontal(|ui| {
                            ui.label("angle");
                            ui.add(egui::DragValue::new(&mut transform.rotate).speed(0.1));
                        });
                    });
                }
                if let Some(velocity) = registry.entities.get_mut::<CVelocity2D>(&entity) {
                    ui.collapsing("Velocity", |ui| {
                        if ui.button("del").clicked() {
                            self.to_remove.push(Box::new(|registry, entity: VersionedIndex| {
                                registry.entities.remove::<CVelocity2D>(&entity);
                            }))
                        }

                        ui.horizontal(|ui| {
                            ui.label("x");
                            ui.add(egui::DragValue::new(&mut velocity.x).speed(1.0));
                            ui.label("y");
                            ui.add(egui::DragValue::new(&mut velocity.y).speed(1.0));
                        });
                    });
                }
                if let Some(rect) = registry.entities.get_mut::<CQuad>(&entity) {
                    ui.collapsing("Quad", |ui| {
                        if ui.button("del").clicked() {
                            self.to_remove.push(Box::new(|registry, entity: VersionedIndex| {
                                registry.entities.remove::<CQuad>(&entity);
                            }))
                        }

                        ui.horizontal(|ui| {
                            ui.label("center x");
                            ui.add(egui::DragValue::new(&mut rect.center_x).speed(1.0));
                            ui.label("center y");
                            ui.add(egui::DragValue::new(&mut rect.center_y).speed(1.0));
                        });
                        ui.horizontal(|ui| {
                            ui.label("width");
                            ui.add(egui::DragValue::new(&mut rect.width).speed(1.0));
                            ui.label("height");
                            ui.add(egui::DragValue::new(&mut rect.height).speed(1.0));
                        });
                    });
                }
            });
        }
    }

    fn add_component(
        &self,
        ui: &mut Ui,
        entity: VersionedIndex,
        registry: &mut Registry
    ) {
        ui.menu_button("Add component", |ui| {
            if registry.entities.get::<CVelocity2D>(&entity).is_none() {
                if ui.button("CVelocity").clicked() {
                    registry.entities.add(&entity, CVelocity2D::default());
                }
            } else { ui.label("CVelocity"); }

            if registry.entities.get::<CQuad>(&entity).is_none() {
                if ui.button("CQuad").clicked() {
                    registry.entities.add(&entity, CQuad::default());
                }
            } else { ui.label("CQuad"); }
        });
    }
}
