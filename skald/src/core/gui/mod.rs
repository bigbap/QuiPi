pub struct GUI {
    ctx: egui::Context
}

impl GUI {
    pub fn new() -> Result<GUI, Box<dyn std::error::Error>> {
        Ok(Self {
            ctx: egui::Context::default()
        })
    }

    pub fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let raw_input: egui::RawInput = egui::RawInput::default();

        let full_output = self.ctx.run(raw_input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add(egui::Label::new("Hello World!"));
                ui.label("A shorter and more convenient way to add a label.");
                if ui.button("Click me").clicked() {
                    // take some action here
                }
            });
        });

        let clipped_primatives = self.ctx.tessellate(full_output.shapes, full_output.pixels_per_point);
        // egui::paint(full_output.textures_delta, clipped_primatives);

        Ok(())
    }
}
