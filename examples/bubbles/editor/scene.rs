use egui::{Context, Vec2};
use quipi::{prelude::qp_editor::IGuiController, world::World};

use crate::{
    qp_schemas::{save_scene_2d, SchemaSprite},
    qp_storage::components::{CScene, CSprite, CTag},
    Schema,
};

use super::components::EntityEditor;

pub struct SceneEditor {
    entity_editor: EntityEditor,
}

impl SceneEditor {
    pub fn new() -> Self {
        Self {
            entity_editor: EntityEditor::new(),
        }
    }
}

impl IGuiController for SceneEditor {
    fn update(&mut self, ctx: &Context, world: &mut World) {
        self.entity_editor.update(ctx, world);
        let entities = world.registry.entity_manager.query_all::<CSprite>();

        egui::Window::new("Scene").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("create entity").clicked() {
                    let schema = SchemaSprite::default();

                    if let Err(e) = schema.build_entity(&mut world.registry) {
                        println!("could not add entity: {}", e);
                    }
                }
                if ui.button("save scene").clicked() {
                    let scenes = world.registry.entity_manager.query_all::<CScene>();
                    let Some(scene_id) = scenes.first() else {
                        return;
                    };

                    if let Some(scene) = world.registry.entity_manager.get::<CScene>(scene_id) {
                        let scene_name = world.registry.strings().get_string(scene.id).unwrap();
                        if let Err(e) = save_scene_2d(&scene_name, *scene_id, &world.registry) {
                            println!("there was a problem saving scene {}: {:?}", scene_name, e);
                        }
                    }
                }
            });

            ui.add_space(10.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_width(200.0);
                for entity in entities.iter() {
                    ui.horizontal(|ui| {
                        let default = CTag {
                            tag: entity.to_string(),
                        };
                        let tag = world
                            .registry
                            .entity_manager
                            .get::<CTag>(entity)
                            .unwrap_or(&default);

                        ui.selectable_value(
                            &mut self.entity_editor.active_entity,
                            Some(*entity),
                            tag.tag.clone(),
                        );
                        if ui.button("x").clicked() {
                            if self.entity_editor.active_entity == Some(*entity) {
                                self.entity_editor.active_entity = None;
                            }
                            world.registry.entity_manager.set_to_delete(*entity);
                        }
                    });
                    ui.allocate_space(Vec2::new(0.0, 5.0));
                }
            });
        });
    }
}
