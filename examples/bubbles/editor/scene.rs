use egui::{Context, Vec2};
use quipi::prelude::qp_data::FrameState;
use quipi::prelude::qp_editor::IGuiController;

use crate::{
    qp_ecs::components::{
        CScene,
        CSprite,
        CTag,
    },
    qp_schemas::{
        SchemaSprite,
        save_scene_2d
    },
    qp_data::ISchema,
    GlobalRegistry
};

use super::components::EntityEditor;

pub struct SceneEditor {
    entity_editor: EntityEditor
}

impl SceneEditor {
    pub fn new() -> Self {
        Self {
            entity_editor: EntityEditor::new()
        }
    }
}

impl IGuiController for SceneEditor {
    fn update(
        &mut self,
        ctx: &Context,
        frame_state: &mut FrameState,
        registry: &mut GlobalRegistry
    ) {
        self.entity_editor.update(ctx, frame_state, registry);
        let entities = registry.entity_manager.query_all::<CSprite>();
    
        egui::Window::new("Scene").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("create entity").clicked() {
                    let schema = SchemaSprite::default();
    
                    if let Err(e) = schema.build_entity(registry) {
                        println!("could not add entity: {}", e);
                    }
                }
                if ui.button("save scene").clicked() {
                    let scenes = registry.entity_manager.query_all::<CScene>();
                    let Some(scene_id) = scenes.first() else { return };
    
                    if let Some(scene) = registry.entity_manager.get::<CScene>(scene_id) {
                        let scene_name = registry.strings()
                            .get_string(scene.id)
                            .unwrap();
                        if let Err(e) = save_scene_2d(&scene_name, *scene_id, &registry) {
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
                        let default = CTag { tag: entity.to_string() };
                        let tag = registry.entity_manager.get::<CTag>(entity)
                            .unwrap_or(&default);

                        ui.selectable_value(
                            &mut self.entity_editor.active_entity,
                            Some(*entity),
                            tag.tag.clone()
                        );
                        if ui.button("x").clicked() {
                            if self.entity_editor.active_entity == Some(*entity) {
                                self.entity_editor.active_entity = None;
                            }
                            registry.entity_manager.set_to_delete(*entity);
                        }
                    });
                    ui.allocate_space(Vec2::new(0.0, 5.0));
                }
            });
        });
    }
}
