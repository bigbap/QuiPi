use egui::{
    epaint::Primitive,
    Mesh, TextureId, TexturesDelta
};

use crate::{gfx::{
    ElementArrayMesh,
    draw::{
        draw_ebo,
        DrawMode
    }
}, components::CCamera};

use super::ShaderProgram;

pub struct GUI {
    ctx: egui::Context,
    mesh: Option<ElementArrayMesh>,
    texture: Option<TextureId>,
    shader: Option<ShaderProgram>,
    camera: CCamera
}

impl GUI {
    pub fn new() -> Result<GUI, Box<dyn std::error::Error>> {
        let shader = ShaderProgram::new("assets/shaders/egui")?;
        let camera = CCamera::new_orthographic(0.0, 800.0, 0.0, 600.0, 0.0, 0.2)?;

        Ok(Self {
            ctx: egui::Context::default(),
            mesh: None,
            texture: None,
            shader: Some(shader),
            camera
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

        let mut clipped_primatives = self.ctx.tessellate(
            full_output.shapes,
            full_output.pixels_per_point
        );

        if let Primitive::Mesh(mesh) = &mut clipped_primatives[0].primitive {
            let (points, colors, uv_coords) = parse_vertices(mesh);

            self.texture = Some(mesh.texture_id);

            let mesh = ElementArrayMesh::new(&mesh.indices)?;
            mesh
                .create_vbo_at(&points, 0, 2)?
                .create_vbo_at(&colors, 1, 4)?
                .create_vbo_at(&uv_coords, 2, 2)?;

            self.mesh = Some(mesh);
        }
        self.paint(full_output.textures_delta);

        Ok(())
    }

    pub fn paint(&self, t_delta: TexturesDelta) {
        if let (Some(mesh), Some(shader)) = (&self.mesh, &self.shader) {
            unsafe {
                gl::Enable(gl::FRAMEBUFFER_SRGB);
                gl::Enable(gl::SCISSOR_TEST);
                gl::Enable(gl::BLEND);
                gl::BlendFunc(gl::ONE, gl::ONE_MINUS_SRC_ALPHA);
                

                gl::Disable(gl::FRAMEBUFFER_SRGB);
                gl::Disable(gl::SCISSOR_TEST);
                gl::Disable(gl::BLEND);
            }


            shader.use_program();
            // shader.set_float_2("u_screenSize", (width, height));
            shader.set_mat4("u_mvpMatrix", &self.camera.projection_matrix);

            mesh.vao.bind();
            draw_ebo(&mesh.vao, DrawMode::Triangles);
            mesh.vao.unbind();
        }
    }

    fn upload_egui_texture(&mut self, id: egui::TextureId, delta: &egui::epaint::ImageDelta) {

    }
}

fn parse_vertices(mesh: &mut Mesh) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let mut pos = Vec::<f32>::new();
    let mut color = Vec::<f32>::new();
    let mut uv_coords = Vec::<f32>::new();

    for row in &mut mesh.vertices {
        pos.push(row.pos.x);
        pos.push(row.pos.y);

        color.push(row.color.r() as f32 / 255.0);
        color.push(row.color.g() as f32 / 255.0);
        color.push(row.color.b() as f32 / 255.0);
        color.push(row.color.a() as f32 / 255.0);

        uv_coords.push(row.uv.x);
        uv_coords.push(row.uv.y);
    }

    (pos, color, uv_coords)
}
