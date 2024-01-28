use self::renderer::Renderer;

mod renderer;

pub struct GUI {
    ctx: egui::Context,

    renderer: Renderer
}

impl GUI {
    pub fn new(
        width: f32,
        height: f32
    ) -> Result<GUI, Box<dyn std::error::Error>> {
        let ctx = egui::Context::default();

        Ok(Self {
            ctx,
            renderer: Renderer::new(width, height)?
        })
    }

    pub fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let raw_input: egui::RawInput = egui::RawInput::default();

        let full_output = self.ctx.run(raw_input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add(egui::Label::new("Hello World!"));
                ui.label("A shorter and more convenient way to\nadd a label.");
                if ui.button("Click me").clicked() {
                    println!("egui was clicked");
                }
            });
        });

        self.renderer.render(
            &self.ctx,
            full_output
        )?;

        Ok(())
    }
}

