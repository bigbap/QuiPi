#![allow(dead_code)]

use egui::{
    ahash::AHashMap,
    ClippedPrimitive,
    epaint::Primitive,
    Mesh,
    Rect,
    vec2,
    Pos2
};

use crate::{
    wrappers::opengl::{
        draw::*,
        functions::gl_scissor,
        shader::ShaderProgram,
        buffer::BufferUsage,
        capabilities::*,
        textures::{
            ITexture,
            gl_use_texture_unit,
        },
    },
    systems::rendering::{
        texture::*,
        mesh::{
            ShaderLocation,
            ElementArrayMesh,
        },
        canvas
    },
};

pub struct Painter {
    textures: AHashMap<egui::TextureId, Box<dyn ITexture>>,
    shader: ShaderProgram,
    pub screen_rect: Rect,
    pub pixels_per_point: f32,
    pub gl_sync_fence: gl::types::GLsync,
}

impl Painter {
    pub fn new(
        scale: f32
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let shader = ShaderProgram::new("assets/shaders/egui")?;

        let pixels_per_point = scale;
        let (_x, _y, width, height) = canvas::get_dimensions();
        let rect = vec2(width as f32, height as f32) / pixels_per_point;
        let screen_rect = Rect::from_min_size(Default::default(), rect);

        Ok(Self {
            textures: AHashMap::default(),
            shader,
            pixels_per_point,
            screen_rect,
            gl_sync_fence: unsafe { gl::FenceSync(gl::SYNC_GPU_COMMANDS_COMPLETE, 0) },
        })
    }

    pub fn update_screen_rect(&mut self) {
        let (_x, _y, width, height) = canvas::get_dimensions();
        let rect = vec2(width as f32, height as f32) / self.pixels_per_point;
        self.screen_rect = Rect::from_min_size(Default::default(), rect);
    }

    pub fn paint(
        &mut self,
        ctx: &egui::Context,
        full_output: egui::FullOutput
    ) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            gl::PixelStorei(gl::UNPACK_ROW_LENGTH, 0);
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 4);
        }

        let (_x, _y, width, height) = canvas::get_dimensions();
        let pixels_per_point = self.pixels_per_point;

        gl_enable(GLCapability::FrameBufferSRGB);
        gl_enable(GLCapability::AlphaBlending);
        gl_enable(GLCapability::ScissorTest);
        gl_blending_func(GLBlendingFactor::One, GLBlendingFactor::OneMinusSrcAlpha);

        let primatives = ctx.tessellate(
            full_output.shapes,
            full_output.pixels_per_point
        );

        let t_delta = full_output.textures_delta;
        for (texture_id, delta) in &t_delta.set {
            self.upload_egui_texture(*texture_id, delta)?;
        }

        let (self_x, self_y) = (self.screen_rect.width(), self.screen_rect.height());
        for ClippedPrimitive {
            clip_rect,
            primitive
        } in primatives {
            if let Primitive::Mesh(mesh) = &primitive {
                if let Some(texture) = self.textures.get(&mesh.texture_id) {
                    texture.use_texture(0);

                    let clip_min_x = pixels_per_point * clip_rect.min.x;
                    let clip_min_y = pixels_per_point * clip_rect.min.y;
                    let clip_max_x = pixels_per_point * clip_rect.max.x;
                    let clip_max_y = pixels_per_point * clip_rect.max.y;

                    let clip_min_x = clip_min_x.clamp(0.0, self_x);
                    let clip_min_y = clip_min_y.clamp(0.0, self_y);
                    let clip_max_x = clip_max_x.clamp(clip_min_x, width as f32);
                    let clip_max_y = clip_max_y.clamp(clip_min_y, height as f32);
                    let clip_min_x = clip_min_x.round() as i32;
                    let clip_min_y = clip_min_y.round() as i32;
                    let clip_max_x = clip_max_x.round() as i32;
                    let clip_max_y = clip_max_y.round() as i32;

                    // scissor Y coordinate is from the bottom
                    gl_scissor(
                        clip_min_x,
                        height - clip_max_y,
                        clip_max_x - clip_min_x,
                        clip_max_y - clip_min_y,
                    );

                    self.draw_mesh(mesh)?;
                }
            }
        }

        gl_disable(GLCapability::FrameBufferSRGB);
        gl_disable(GLCapability::AlphaBlending);
        gl_disable(GLCapability::ScissorTest);

        Ok(())
    }

    fn draw_mesh(
        &self,
        mesh: &Mesh,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
        self.shader.set_float_2("u_screenSize", (self.screen_rect.width(), self.screen_rect.height()));

        m_mesh.vao.bind();
        gl_use_texture_unit(0);
        gl_draw(DrawBuffer::Elements, DrawMode::Triangles, m_mesh.vao.count());
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
