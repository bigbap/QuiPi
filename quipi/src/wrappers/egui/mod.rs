use egui::{
    RawInput,
    Pos2,
    Ui
};
use sdl2::{
    event::{
        Event,
        WindowEvent
    },
    keyboard::Keycode
};

use crate::{
    engine::AppState,
    wrappers::egui::{
        painter::Painter,
        input::parse_event,
    },
    FrameResponse,
    systems::rendering::canvas::set_dimensions
};

mod painter;
mod input;

pub struct UiRegion {
    pub name: String,
    pub resizable: bool,
    pub cb: Box<dyn Fn(&mut Ui)>
}

pub struct GUI {
    ctx: egui::Context,
    painter: Painter,
    raw_input: RawInput,

    windows: Vec<UiRegion>
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
            windows: vec![]
        })
    }

    pub fn add_ui_region(
        &mut self,
        name: &str,
        resizable: bool,
        cb: impl Fn(&mut Ui) + 'static
    ) {
        self.windows.push(UiRegion {
            name: name.to_string(),
            resizable,
            cb: Box::new(cb)
        });
    }

    pub fn update(
        &mut self,
        app_state: &mut AppState
    ) -> Result<FrameResponse, Box<dyn std::error::Error>> {
        if !app_state.editor_mode {
            return Ok(FrameResponse::Ignore)
        }

        let _pos = self.ctx.input(|i| i.pointer.hover_pos()).unwrap_or(Pos2::new(-1.0, -1.0));
        self.ctx.begin_frame(self.raw_input.take());
        
        for UiRegion { name, resizable, cb, ..} in self.windows.iter() {
            egui::Window::new(name)
                .resizable(*resizable)
                .show(&self.ctx, cb);
        }

        let full_output = self.ctx.end_frame();

        self.painter.paint(
            &self.ctx,
            full_output
        )?;

        self.handle_input(app_state)
    }

    fn handle_input(
        &mut self,
        app_state: &mut AppState
    ) -> Result<FrameResponse, Box<dyn std::error::Error>> {
        for event in app_state.events.iter() {
            match event {
                Event::Quit { .. } => return Ok(FrameResponse::Quit),
                Event::KeyDown { keycode: Some(Keycode::F12), .. } => {
                    app_state.editor_mode = false;

                    return Ok(FrameResponse::Ignore);
                },
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
        
        Ok(FrameResponse::Ignore)
    }
}

