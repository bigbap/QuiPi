use egui::{epaint::Primitive, Mesh};

use crate::{gfx::ElementArrayMesh, components::CModelNode};

use super::ShaderProgram;

pub struct GUI {
    ctx: egui::Context,
    mesh: Option<ElementArrayMesh>,
    shader: Option<ShaderProgram>
}

impl GUI {
    pub fn new() -> Result<GUI, Box<dyn std::error::Error>> {
        let shader = ShaderProgram::new("assets/shaders/egui")?;

        Ok(Self {
            ctx: egui::Context::default(),
            mesh: None,
            shader: Some(shader)
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

        let mut clipped_primatives = self.ctx.tessellate(full_output.shapes, full_output.pixels_per_point);

        if let Primitive::Mesh(mesh) = &mut clipped_primatives[0].primitive {
            let (points, colors) = parse_vertices(mesh);

            let mesh = ElementArrayMesh::new(&mesh.indices)?;
            mesh
                .create_vbo_at(&points, 0, 2)?
                .create_vbo_at(&colors, 1, 4)?;

            self.mesh = Some(mesh);
        }
        // egui::paint(full_output.textures_delta, clipped_primatives);

        Ok(())
    }
}

fn parse_vertices(mesh: &mut Mesh) -> (Vec<f32>, Vec<f32>) {
    let mut pos = Vec::<f32>::new();
    let mut color = Vec::<f32>::new();

    for row in &mut mesh.vertices {
        pos.push(row.pos.x);
        pos.push(row.pos.y);

        color.push(row.color.r() as f32 / 255.0);
        color.push(row.color.g() as f32 / 255.0);
        color.push(row.color.b() as f32 / 255.0);
        color.push(row.color.a() as f32 / 255.0);
    }

    (pos, color)
}
