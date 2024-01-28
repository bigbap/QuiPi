use egui::{
    epaint::Primitive,
    Mesh,
    ahash::AHashMap, ClippedPrimitive
};

use crate::{
    gfx::{
        gl_draw,
        ElementArrayMesh,
        draw::{DrawMode, DrawBuffer},
        mesh::{
            BufferUsage,
            ShaderLocation
        }, texture::{
            ITexture,
            gl_use_texture_unit,
            from_buffer_rgba
        }
    },
    components::CCamera
};

use super::ShaderProgram;

pub struct GUI {
    ctx: egui::Context,
    textures: AHashMap<egui::TextureId, Box<dyn ITexture>>,
    shader: Option<ShaderProgram>,
    camera: CCamera
}

impl GUI {
    pub fn new() -> Result<GUI, Box<dyn std::error::Error>> {
        let shader = ShaderProgram::new("assets/shaders/egui")?;
        let camera = CCamera::new_orthographic(0.0, 800.0, 600.0, 0.0, 0.0, 0.2)?;
        let ctx = egui::Context::default();

        Ok(Self {
            ctx,
            textures: AHashMap::default(),
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

        self.paint(full_output)?;

        Ok(())
    }

    pub fn paint(
        &mut self,
        full_output: egui::FullOutput
    ) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            gl::Enable(gl::FRAMEBUFFER_SRGB);
            gl::Enable(gl::SCISSOR_TEST);
            gl::Enable(gl::BLEND);
        }

        let primatives = self.ctx.tessellate(
            full_output.shapes,
            full_output.pixels_per_point
        );

        let t_delta = full_output.textures_delta;
        for (texture_id, delta) in &t_delta.set {
            self.upload_egui_texture(*texture_id, delta)?;
        }

        for ClippedPrimitive {
            clip_rect,
            primitive
        } in primatives {
            if let Primitive::Mesh(mesh) = &primitive {
                if let Some(texture) = self.textures.get(&mesh.texture_id) {
                    texture.use_texture(0);

                    let clip_min_x = 1.0 * clip_rect.min.x;
                    let clip_min_y = 1.0 * clip_rect.min.y;
                    let clip_max_x = 1.0 * clip_rect.max.x;
                    let clip_max_y = 1.0 * clip_rect.max.y;
                    let clip_min_x = clip_min_x.clamp(0.0, 800.0);
                    let clip_min_y = clip_min_y.clamp(0.0, 600.0);
                    let clip_max_x = clip_max_x.clamp(clip_min_x, 800.0);
                    let clip_max_y = clip_max_y.clamp(clip_min_y, 600.0);
                    let clip_min_x = clip_min_x.round() as i32;
                    let clip_min_y = clip_min_y.round() as i32;
                    let clip_max_x = clip_max_x.round() as i32;
                    let clip_max_y = clip_max_y.round() as i32;

                    //scissor Y coordinate is from the bottom
                    unsafe {
                        gl::Scissor(
                            clip_min_x,
                            600 - clip_max_y,
                            clip_max_x - clip_min_x,
                            clip_max_y - clip_min_y,
                        );
                    }

                    self.draw_mesh(mesh)?;
                }
            }
        }

        unsafe {
            gl::Disable(gl::FRAMEBUFFER_SRGB);
            gl::Disable(gl::SCISSOR_TEST);
            // gl::Disable(gl::BLEND);
        }

        Ok(())
    }

    fn draw_mesh(&self, mesh: &Mesh) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(shader) = &self.shader {
            let (points, colors, uv_coords) = parse_vertices(mesh);
            let mut m_mesh = ElementArrayMesh::new(
                mesh.indices.len(),
                BufferUsage::StreamDraw
            )?;
            m_mesh
                .with_ebo(&mesh.indices)?
                .with_vbo::<2, f32>(ShaderLocation::Zero, &points)?
                .with_vbo::<4, f32>(ShaderLocation::One, &colors)?
                .with_vbo::<2, f32>(ShaderLocation::Two, &uv_coords)?;

            shader.use_program();
            shader.set_mat4("u_mvpMatrix", &self.camera.projection_matrix);

            m_mesh.vao.bind();
            gl_use_texture_unit(0);
            gl_draw(DrawBuffer::Elements, DrawMode::Triangles, m_mesh.vao.count());
            m_mesh.vao.unbind();
        }

        Ok(())
    }

    fn upload_egui_texture(
        &mut self,
        id: egui::TextureId,
        delta: &egui::epaint::ImageDelta
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Modeled after equi_sdl2_gl's upload_egui_texture.
        // https://github.com/ArjunNair/egui_sdl2_gl/blob/main/src/painter.rs

        let pixels: Vec<u8> = match &delta.image {
            egui::ImageData::Color(image) => {
                assert_eq!(
                    image.width() * image.height(),
                    image.pixels.len(),
                    "mismatch between texture size and texel count"
                );

                image
                    .pixels
                    .iter()
                    .flat_map(|color| color.to_array())
                    .collect()
            },
            egui::ImageData::Font(image) => image
                .srgba_pixels(None)
                .flat_map(|color| color.to_array())
                .collect()
        };

        let t_width = delta.image.width();
        let t_height = delta.image.height();

        if let (Some(patch_pos), Some(texture)) = (
            delta.pos,
            self.textures.get_mut(&id)
        ) {
            println!("got here");
            
        } else {
            let texture = from_buffer_rgba(
                t_width as i32,
                t_height as i32,
                &pixels
            )?;

            self.textures.insert(id, texture);
        }

        Ok(())
    }
}

fn parse_vertices(mesh: &Mesh) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let mut pos = Vec::<f32>::new();
    let mut color = Vec::<f32>::new();
    let mut uv_coords = Vec::<f32>::new();

    for row in &mesh.vertices {
        pos.push(row.pos.x);
        pos.push(row.pos.y);

        color.push(row.color.r() as f32);
        color.push(row.color.g() as f32);
        color.push(row.color.b() as f32);
        color.push(row.color.a() as f32);

        uv_coords.push(row.uv.x);
        uv_coords.push(row.uv.y);
    }

    (pos, color, uv_coords)
}
