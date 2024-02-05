use egui::{
    RawInput,
    Ui
};
use sdl2::event::{
    Event,
    WindowEvent
};

use crate::{
    engine::AppState,
    wrappers::egui::{
        painter::Painter,
        input::parse_event,
    },
    systems::rendering::canvas::set_dimensions
};

mod painter;
mod input;

pub struct GUI {
    ctx: egui::Context,
    painter: Painter,
    raw_input: RawInput,
}

impl GUI {
    pub fn new(
        scale: f32
    ) -> Result<GUI, Box<dyn std::error::Error>> {
        let ctx = egui::Context::default();
        let painter = Painter::new(scale)?;
        let raw_input = egui::RawInput::default();

        Ok(Self {
            ctx,
            painter,
            raw_input,
        })
    }

    pub fn begin_frame(&mut self) {
        self.ctx.begin_frame(self.raw_input.take());
    }
    pub fn add_panel_top(
        &mut self,
        name: &str,
        cb: impl FnOnce(&mut Ui)
    ) {
        egui::TopBottomPanel::top(name.to_string())
            .show(&self.ctx, cb);
    }
    pub fn add_window(
        &mut self,
        name: &str,
        cb: impl FnOnce(&mut Ui)
    ) {
        egui::Window::new(name.to_string())
            .resizable(true)
            .show(&self.ctx, cb);
    }
    pub fn end_frame(
        &mut self,
        app_state: &AppState
    ) -> Result<(), Box<dyn std::error::Error>> {
        let full_output = self.ctx.end_frame();

        self.painter.paint(
            &self.ctx,
            full_output
        )?;

        self.handle_input(app_state)
    }

    fn handle_input(
        &mut self,
        app_state: &AppState
    ) -> Result<(), Box<dyn std::error::Error>> {
        for event in app_state.events.iter() {
            match event {
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(width, height) | WindowEvent::SizeChanged(width, height) => {
                        set_dimensions(0, 0, *width, *height);
                        self.painter.update_screen_rect();
                        self.raw_input.screen_rect = Some(self.painter.screen_rect);
                    },
                    _ => ()
                },
                _ => {
                    if let Some(parsed) = parse_event(event, &self.painter) {
                        self.raw_input.events.push(parsed);
                    }
                }
            }
        }
        
        Ok(())
    }
}

