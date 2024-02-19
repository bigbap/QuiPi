#![allow(dead_code)]

/*
* Modeled after equi_sdl2_gl's upload_egui_texture.
* https://github.com/ArjunNair/egui_sdl2_gl/blob/main/src/painter.rs
*/

use egui::{
    ahash::AHashMap,
    ClippedPrimitive,
    epaint::Primitive,
    Mesh,
    Rect,
    vec2,
};

use crate::{
    core::{
        rendering::texture::*,
        canvas
    },
    opengl::buffer::{
        create_ebo,
        vertex_attribute_pointer,
        Buffer,
        VertexArray,
        VBO
    },
    platform::opengl::{
        buffer::BufferUsage, capabilities::*, draw::*, functions::gl_scissor, shader::ShaderProgram, textures::{
            gl_use_texture_unit, Format, ParameterName, ParameterValue, Texture
        }
    }
};

pub struct Painter {
    textures: AHashMap<egui::TextureId, Texture>,
    shader: ShaderProgram,
    pub screen_rect: Rect,
    pub pixels_per_point: f32,
    pub gl_sync_fence: gl::types::GLsync,
}

impl Painter {
    pub fn new(
        scale: f32
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let shader = ShaderProgram::from_str(VERT_SHADER, FRAG_SHADER)?;

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

        let vao = VertexArray::new();
        let ebo = create_ebo(&mesh.indices, &BufferUsage::StaticDraw);

        vao.bind();
        ebo.bind();

        let vbo_points = Buffer::<VBO>::new();
        let vbo_colors = Buffer::<VBO>::new();
        let vbo_uv_coords = Buffer::<VBO>::new();

        vbo_points.bind();
        vbo_points.buffer_data::<f32>(mesh.indices.len() * 2, Some(&points), &BufferUsage::StreamDraw);
        vertex_attribute_pointer(0, 2, std::mem::size_of::<f32>() * 2, 0);

        vbo_colors.bind();
        vbo_colors.buffer_data::<f32>(mesh.indices.len() * 4, Some(&colors), &BufferUsage::StreamDraw);
        vertex_attribute_pointer(1, 4, std::mem::size_of::<f32>() * 4, 0);

        vbo_uv_coords.bind();
        vbo_uv_coords.buffer_data::<f32>(mesh.indices.len() * 2, Some(&uv_coords), &BufferUsage::StreamDraw);
        vertex_attribute_pointer(2, 2, std::mem::size_of::<f32>() * 2, 0);

        self.shader.use_program();
        self.shader.set_float_2("u_screenSize", (self.screen_rect.width(), self.screen_rect.height()));

        vao.bind();
        gl_use_texture_unit(0);
        gl_draw(DrawBuffer::Elements, DrawMode::Triangles, mesh.indices.len() as i32);
        vao.unbind();

        Ok(())
    }

    fn upload_egui_texture(
        &mut self,
        id: egui::TextureId,
        delta: &egui::epaint::ImageDelta
    ) -> Result<(), Box<dyn std::error::Error>> {
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
            let patch_x = patch_pos[0];
            let patch_y = patch_pos[1];
            let patch_width = t_width;
            let patch_height = t_height;
            
            texture.bind().sub_image_data(
                patch_x as i32,
                patch_y as i32,
                patch_width as i32,
                patch_height as i32,
                Format::Rgba,
                &pixels
            );
            
        } else {
            let texture = from_buffer_rgba(
                t_width as i32,
                t_height as i32,
                &pixels
            );

            texture.bind()
                .set_parameter(ParameterName::WrapS, ParameterValue::ClampToEdge)
                .set_parameter(ParameterName::WrapT, ParameterValue::ClampToEdge)
                .set_parameter(ParameterName::MinFilter, ParameterValue::Linear)
                .set_parameter(ParameterName::MagFilter, ParameterValue::Linear);


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

// copied from https://github.com/ArjunNair/egui_sdl2_gl/tree/main
const VERT_SHADER: &str = r#"
#version 450 core

layout (location = 0) in vec2 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec2 aUVCoords; 

uniform vec2 u_screenSize;

out vec4 color;
out vec2 uvCoords;

// 0-1 linear  from  0-255 sRGB
vec3 linearFromSrgb(vec3 srgb) {
    bvec3 cutoff = lessThan(srgb, vec3(10.31475));
    vec3 lower = srgb / vec3(3294.6);
    vec3 higher = pow((srgb + vec3(14.025)) / vec3(269.025), vec3(2.4));
    return mix(higher, lower, cutoff);
}

vec4 linearFromSrgba(vec4 srgba) {
    return vec4(linearFromSrgb(srgba.rgb), srgba.a / 255.0);
}

void main(){
    gl_Position = vec4(
        2.0 * aPos.x / u_screenSize.x - 1.0,
        1.0 - 2.0 * aPos.y / u_screenSize.y,
        0.0,
        1.0
    );

    color = linearFromSrgba(aColor);
    uvCoords = aUVCoords;
}
"#;

// copied from https://github.com/ArjunNair/egui_sdl2_gl/tree/main
const FRAG_SHADER: &str = r#"
#version 450 core

in vec4 color;
in vec2 uvCoords;

uniform sampler2D u_sampler;

out vec4 fragColor;

// 0-255 sRGB  from  0-1 linear
vec3 srgbFromLinear(vec3 rgb) {
    bvec3 cutoff = lessThan(rgb, vec3(0.0031308));
    vec3 lower = rgb * vec3(3294.6);
    vec3 higher = vec3(269.025) * pow(rgb, vec3(1.0 / 2.4)) - vec3(14.025);
    return mix(higher, lower, vec3(cutoff));
}

vec4 srgbaFromLinear(vec4 rgba) {
    return vec4(srgbFromLinear(rgba.rgb), 255.0 * rgba.a);
}

vec3 linearFromSrgb(vec3 srgb) {
    bvec3 cutoff = lessThan(srgb, vec3(10.31475));
    vec3 lower = srgb / vec3(3294.6);
    vec3 higher = pow((srgb + vec3(14.025)) / vec3(269.025), vec3(2.4));
    return mix(higher, lower, vec3(cutoff));
}

vec4 linearFromSrgba(vec4 srgba) {
    return vec4(linearFromSrgb(srgba.rgb), srgba.a / 255.0);
}

void main() {
    vec4 textureRgba = linearFromSrgba(texture(u_sampler, uvCoords) * 255.0);
    fragColor = color * textureRgba;
}  
"#;
