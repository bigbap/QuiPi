use crate::engine::{
    AppState,
    InputOwner
};
use self::{
    renderer::Renderer,
    // input::parse_input
};

mod renderer;
mod input;

pub struct GUI {
    ctx: egui::Context,

    renderer: Renderer
}

impl GUI {
    pub fn new(
        scale: f32
    ) -> Result<GUI, Box<dyn std::error::Error>> {
        let ctx = egui::Context::default();

        Ok(Self {
            ctx,
            renderer: Renderer::new(scale)?
        })
    }

    pub fn update(
        &mut self,
        app_state: &AppState
    ) -> Result<(), Box<dyn std::error::Error>> {
        if app_state.input_owner != InputOwner::Editor {
            return Ok(())
        }

        // let raw_input = egui::RawInput::default();
        let raw_input = egui::RawInput {
            screen_rect: Some(self.renderer.screen_rect),
            ..Default::default()
        };

        self.ctx.begin_frame(raw_input);
        egui::CentralPanel::default().show(&self.ctx, |ui| {
            ui.add(egui::Label::new("Hello World!"));
            ui.label("A shorter and more convenient way to\nadd a label.");
            if ui.button("Click me").clicked() {
                println!("egui was clicked");
            }
        });
        let full_output = self.ctx.end_frame();

        self.renderer.render(
            &self.ctx,
            full_output
        )?;

        Ok(())
    }
}

