use std::collections::HashMap;
use crate::{gfx::{
    ElementArrayMesh,
    mesh::BufferUsage,
    texture::gl_use_texture_unit,
    gl_draw, draw::{DrawBuffer, DrawMode}
}, utils::to_abs_path};

use super::ShaderProgram;

mod characters;

pub static DEFAULT_FONT: &str = "assets/fonts/FiraSansRegular.ttf";

#[derive(Debug)]
pub struct TextRenderer {
    shader: ShaderProgram,
    mesh: ElementArrayMesh,
    char_map: HashMap<char, characters::Character>,

    pub color: glm::Vec3,
    pub scale: f32,
}

impl TextRenderer {
    pub fn new(
        font: &str,
        width: f32,
        height: f32
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let char_map = characters::load_char_textures(&to_abs_path(font)?)?;
        let shader = ShaderProgram::from_str(
            VERT_SHADER,
            FRAG_SHADER,
        )?;

        shader.set_mat4("projection", &glm::ortho(0.0, width, 0.0, height, 0.0, 0.2));
        
        let mut mesh = ElementArrayMesh::new(6, BufferUsage::DynamicDraw)?;
        mesh.create_vbo::<4, f32>(0, 6 * 4, None)?;

        Ok(Self {
            shader,
            mesh,
            char_map,

            color: glm::vec3(0.0, 0.0, 0.0),
            scale: 1.0
        })
    }

    pub fn draw(
        &self,
        text: String,
        mut pos: glm::Vec2,
    ) {
        self.shader.use_program();
        self.shader.set_float_3("textColor", (self.color.x, self.color.y, self.color.z));
        
        gl_use_texture_unit(0);

        self.mesh.vao.bind();

        for c in text.chars() {
            let Some(ch) = self.char_map.get(&c) else { continue; };

            let x_pos = pos.x + ch.bearing.x * self.scale;
            let y_pos = pos.y - (ch.size.y - ch.bearing.y) * self.scale;

            let w = ch.size.x * self.scale;
            let h = ch.size.y * self.scale;

            let vertices = [
                x_pos,      y_pos + h,  0.0, 0.0,
                x_pos,      y_pos,      0.0, 1.0,
                x_pos + w,  y_pos,      1.0, 1.0,
                x_pos,      y_pos + h,  0.0, 0.0,
                x_pos + w,  y_pos,      1.0, 1.0,
                x_pos + w,  y_pos + h,  1.0, 0.0
            ];

            ch.texture.use_texture(0);
            if let Some(mesh) = self.mesh.vbo_list.get(0) {
                mesh.bind();
                mesh.buffer_sub_data(
                    0,
                    vertices.len(),
                    Some(&vertices)
                );
                mesh.unbind();

                gl_draw(
                    DrawBuffer::Arrays,
                    DrawMode::Triangles,
                    6
                );
            }

            pos.x += (ch.advance >> 6) as f32 * self.scale;
        }

        self.mesh.vao.unbind();
    }
}

// copied from https://learnopengl.com/In-Practice/Text-Rendering
const VERT_SHADER: &str = r#"
#version 450 core
layout (location = 0) in vec4 vertex; // <vec2 pos, vec2 tex>
out vec2 TexCoords;

uniform mat4 projection;

void main()
{
    gl_Position = projection * vec4(vertex.xy, 0.0, 1.0);
    TexCoords = vertex.zw;
}
"#;

// copied from https://learnopengl.com/In-Practice/Text-Rendering
const FRAG_SHADER: &str = r#"
#version 450 core
in vec2 TexCoords;
out vec4 color;

uniform sampler2D text;
uniform vec3 textColor;

void main()
{    
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(text, TexCoords).r);
    color = vec4(textColor, 1.0) * sampled;
}  
"#;
