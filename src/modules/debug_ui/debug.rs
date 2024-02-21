use crate::{
    FrameState,
    Registry,
    GUI
};

pub fn debug(
    gui: &GUI,
    app_state: &FrameState,
    registry: &Registry
) {
    egui::Window::new("Debug Info")
        .show(&gui.ctx, |ui| {
            ui.set_width(200.0);
            ui.label(format!("fps: {}", app_state.debug_info.fps));
            ui.label(format!("ms (frame): {}", app_state.debug_info.ms));
            ui.label(format!("ms (draw time): {}", app_state.render_info.total_ms));
            ui.label(format!("ms (editor time): {}", app_state.editor_info.ms));
            ui.label(format!("draw calls: {}", app_state.render_info.num_draw_calls));
            ui.separator();
            ui.label(format!("entity count: {}", registry.entities.count()));
            ui.label(format!("allocator size: {}", registry.entities.allocator_size()));
        });
}