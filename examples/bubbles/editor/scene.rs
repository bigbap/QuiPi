use egui::Vec2;

use crate::{
    GUI,
    ecs::components::{
        CScene,
        CSprite,
        CTag,
    },
    schemas::{
        SchemaSprite,
        save_scene_2d
    },
    data::ISchema,
    Registry
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

    pub fn update(
        &mut self,
        gui: &GUI,
        registry: &mut Registry
    ) {
        self.entity_editor.update(gui, registry);
        let entities = registry.entities.query_all::<CSprite>();
    
        egui::Window::new("Scene").show(&gui.ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("create entity").clicked() {
                    let schema = SchemaSprite::default();
    
                    if let Err(e) = schema.build_entity(registry) {
                        println!("could not add entity: {}", e);
                    }
                }
                if ui.button("save scene").clicked() {
                    let scenes = registry.entities.query_all::<CScene>();
                    let Some(scene_id) = scenes.first() else { return };
    
                    if let Some(scene) = registry.entities.get::<CScene>(scene_id) {
                        let scene_name = registry.string_interner.get_string(scene.id).unwrap();
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
                        let tag = registry.entities.get::<CTag>(entity)
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
                            registry.entities.set_to_delete(*entity);
                        }
                    });
                    ui.allocate_space(Vec2::new(0.0, 5.0));
                }
            });
        });
    }
}
