use egui::Context;
use crate::{
    qp_data::FrameState,
    qp_editor::IGuiController,
    GlobalRegistry
};

pub struct DebugUi {}

impl IGuiController for DebugUi {
    fn update(
        &mut self,
        ctx: &Context,
        frame_state: &mut FrameState,
        registry: &mut GlobalRegistry
    ) {
        egui::Window::new("Debug Info")
            .show(ctx, |ui| {
                ui.set_width(200.0);
                ui.label(format!("fps: {}", frame_state.debug_info.fps));
                ui.label(format!("frame time (ms): {}", frame_state.debug_info.frame_ms));
                ui.label(format!("draw time (ms): {}", frame_state.debug_info.render_ms));
                ui.label(format!("editor time (ms): {}", frame_state.debug_info.editor_ms));
                ui.label(format!("draw calls: {}", frame_state.debug_info.draw_calls));
                ui.separator();
                ui.label(format!("entity count: {}", registry.entity_manager.count()));
                ui.label(format!("allocator size: {}", registry.entity_manager.allocator_size()));
            });
    }
}
