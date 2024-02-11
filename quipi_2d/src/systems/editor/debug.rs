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
            ui.label(format!("ms: {}", app_state.debug_info.ms));
            ui.separator();
            ui.label(format!("entity count: {}", registry.entities.count()));
            ui.label(format!("allocator size: {}", registry.entities.allocator_size()));
        });
}