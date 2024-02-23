use crate::{
    data::FrameState,
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
            ui.label(format!("ms (frame): {}", app_state.debug_info.frame_ms));
            ui.label(format!("ms (draw time): {}", app_state.debug_info.render_ms));
            ui.label(format!("ms (editor time): {}", app_state.debug_info.editor_ms));
            ui.label(format!("draw calls: {}", app_state.debug_info.draw_calls));
            ui.separator();
            ui.label(format!("entity count: {}", registry.entities.count()));
            ui.label(format!("allocator size: {}", registry.entities.allocator_size()));
        });
}