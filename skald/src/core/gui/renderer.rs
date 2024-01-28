use egui::{
    ahash::AHashMap,
    ClippedPrimitive,
    epaint::Primitive,
    Mesh
};

use crate::{
    core::ShaderProgram,
    gfx::{
        call_api_draw,
        texture::*,
        mesh::{
            ShaderLocation,
            BufferUsage
        },
        ElementArrayMesh,
        draw::*
    },
    gl_capabilities::{
        self,
        GLCapability,
        BlendingFactor
    }
};

pub struct Renderer {
    textures: AHashMap<egui::TextureId, Box<dyn ITexture>>,
    shader: ShaderProgram,
    projection: glm::Mat4,
}

impl Renderer {
    pub fn new(
        width: f32,
        height: f32
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let shader = ShaderProgram::new("assets/shaders/egui")?;
        let projection = glm::ortho(0.0, width, height, 0.0, 0.0, 0.2);

        Ok(Self {
            textures: AHashMap::default(),
            shader,
            projection
        })

    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        full_output: egui::FullOutput
    ) -> Result<(), Box<dyn std::error::Error>> {
        gl_capabilities::enable(GLCapability::FrameBufferSRGB);
        gl_capabilities::enable(GLCapability::AlphaBlending);
        gl_capabilities::enable(GLCapability::ScissorTest);
        gl_capabilities::blending_func(BlendingFactor::One, BlendingFactor::OneMinusSrcAlpha);

        let primatives = ctx.tessellate(
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

                    let clip_min_x = clip_rect.min.x;
                    let clip_min_y = clip_rect.min.y;
                    let clip_max_x = clip_rect.max.x;
                    let clip_max_y = clip_rect.max.y;
                    let clip_min_x = clip_min_x.clamp(0.0, 400.0);
                    let clip_min_y = clip_min_y.clamp(0.0, 300.0);
                    let clip_max_x = clip_max_x.clamp(clip_min_x, 400.0);
                    let clip_max_y = clip_max_y.clamp(clip_min_y, 300.0);
                    let clip_min_x = clip_min_x.round() as i32;
                    let clip_min_y = clip_min_y.round() as i32;
                    let clip_max_x = clip_max_x.round() as i32;
                    let clip_max_y = clip_max_y.round() as i32;

                    // scissor Y coordinate is from the bottom
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

        gl_capabilities::disable(GLCapability::FrameBufferSRGB);
        gl_capabilities::disable(GLCapability::AlphaBlending);
        gl_capabilities::disable(GLCapability::ScissorTest);

        Ok(())
    }

    fn draw_mesh(&self, mesh: &Mesh) -> Result<(), Box<dyn std::error::Error>> {
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

        self.shader.use_program();
        self.shader.set_mat4("u_mvpMatrix", &self.projection);

        m_mesh.vao.bind();
        gl_use_texture_unit(0);
        call_api_draw(DrawBuffer::Elements, DrawMode::Triangles, m_mesh.vao.count());
        m_mesh.vao.unbind();

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

        if let (Some(_patch_pos), Some(_texture)) = (
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
