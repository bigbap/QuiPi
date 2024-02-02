use egui::RawInput;
use sdl2::{event::{Event, WindowEvent}, keyboard::Keycode};

use crate::{
    engine::{
        AppState,
        InputOwner
    },
    wrappers::egui::input::parse_event,
    FrameResponse, systems::rendering::canvas::set_dimensions
};
use self::{
    painter::Painter,
    // input::parse_input
};

mod painter;
mod input;

pub struct GUI {
    ctx: egui::Context,

    painter: Painter,

    raw_input: RawInput
}

impl GUI {
    pub fn new(
        scale: f32
    ) -> Result<GUI, Box<dyn std::error::Error>> {
        let ctx = egui::Context::default();
        let painter = Painter::new(scale)?;
        let raw_input = egui::RawInput {
            screen_rect: Some(painter.screen_rect),
            ..RawInput::default()
        };

        Ok(Self {
            ctx,
            painter,
            raw_input
        })
    }

    pub fn update(
        &mut self,
        app_state: &mut AppState
    ) -> Result<FrameResponse, Box<dyn std::error::Error>> {
        if app_state.input_owner != InputOwner::Editor {
            return Ok(FrameResponse::Ignore)
        }

        self.ctx.begin_frame(self.raw_input.take());
        egui::SidePanel::left(egui::Id::new(1234)).show(&self.ctx, |ui| {
            ui.add(egui::Label::new("Hello World!"));
            ui.label("This is a label");
            if ui.button("Click me").clicked() {
                println!("egui was clicked");
            }
        });
        let full_output = self.ctx.end_frame();

        self.painter.paint(
            &self.ctx,
            full_output
        )?;

        for event in app_state.winapi.get_event_queue()?.poll_iter() {
            match event {
                Event::Quit { .. } => return Ok(FrameResponse::Quit),
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Ok(FrameResponse::RelinquishInput),
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(width, height) | WindowEvent::SizeChanged(width, height) => {
                        set_dimensions(0, 0, width, height);
                        self.painter.update_screen_rect();
                        self.raw_input.screen_rect = Some(self.painter.screen_rect);
                    },
                    _ => ()
                },
                _ => {
                    if let Some(parsed) = parse_event(&event) {
                        self.raw_input.events.push(parsed);
                    }
                }
            }
        }

        Ok(FrameResponse::Ignore)
    }
}

