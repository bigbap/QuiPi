use crate::qp_editor::IGuiController;
use egui::Context;
use quipi::world::World;

pub struct DebugUi {}

impl IGuiController for DebugUi {
    fn update(&mut self, ctx: &Context, world: &mut World) {
        egui::Window::new("Debug Info").show(ctx, |ui| {
            ui.set_width(200.0);
            ui.label(format!("fps: {}", world.debug_info.fps));
            ui.label(format!("frame time (ms): {}", world.debug_info.frame_ms));
            ui.label(format!("draw time (ms): {}", world.debug_info.render_ms));
            ui.label(format!("editor time (ms): {}", world.debug_info.editor_ms));
            ui.label(format!("draw calls: {}", world.debug_info.draw_calls));
            ui.separator();
            ui.label(format!(
                "entity count: {}",
                world.registry.entity_manager.count()
            ));
            ui.label(format!(
                "allocator size: {}",
                world.registry.entity_manager.allocator_size()
            ));
        });
    }
}
