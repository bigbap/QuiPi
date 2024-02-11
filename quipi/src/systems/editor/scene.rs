use egui::Vec2;

use crate::{
    components::{
        CDrawable, CScene
    },
    schemas::{
        ISchema, SchemaEntity2D
    },
    systems::scene::save_scene_2d,
    wrappers::egui::GUI, Registry
};

use super::components::EntityEditor;

pub struct SceneEditor {
    entity_editor: EntityEditor,
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

        // TODO: this is currently hardcoded
        let entities = registry.entities.query_all::<CDrawable>();
    
        egui::Window::new("Scene").show(&gui.ctx, |ui| {
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
                        ui.selectable_value(
                            &mut self.entity_editor.active_entity,
                            Some(*entity),
                            entity.to_string()
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