use std::collections::HashMap;
use crate::gfx::{
    ElementArrayMesh,
    mesh::{
        BufferUsage,
        VboKind
    },
    texture::gl_use_texture_unit,
    gl_draw, draw::{DrawBuffer, DrawMode}
};

use super::ShaderProgram;

mod characters;

pub struct TextRenderer {
    shader: ShaderProgram,
    mesh: ElementArrayMesh,
    char_map: HashMap<char, characters::Character>,
}

impl TextRenderer {
    pub fn new(
        font: &str,
        projection: glm::Mat4
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let char_map = characters::load_char_textures(font)?;
        let shader = ShaderProgram::from_str(
            VERT_SHADER,
            FRAG_SHADER,
        )?;

        shader.set_mat4("projection", &projection);
        
        let mut mesh = ElementArrayMesh::new(6, BufferUsage::DynamicDraw)?;
        mesh.create_vbo::<0, 4, f32>(VboKind::Vertex, 6 * 4, None)?;

        Ok(Self {
            shader,
            mesh,
            char_map,
        })
    }

    pub fn draw(
        &self,
        text: String,
        mut pos: glm::Vec2,
        color: glm::Vec3,
        scale: f32
    ) {
        self.shader.use_program();
        self.shader.set_float_3("textColor", (color.x, color.y, color.z));
        
        gl_use_texture_unit(0);

        self.mesh.vao.bind();

        for c in text.chars() {
            let Some(ch) = self.char_map.get(&c) else { continue; };

            let x_pos = pos.x + ch.bearing.x * scale;
            let y_pos = pos.y - (ch.size.y - ch.bearing.y) * scale;

            let w = ch.size.x * scale;
            let h = ch.size.y * scale;

            let vertices = [
                x_pos,      y_pos + h,  0.0, 0.0,
                x_pos,      y_pos,      0.0, 1.0,
                x_pos + w,  y_pos,      1.0, 1.0,
                x_pos,      y_pos + h,  0.0, 0.0,
                x_pos + w,  y_pos,      1.0, 1.0,
                x_pos + w,  y_pos + h,  1.0, 0.0
            ];

            ch.texture.use_texture(0);
            if let Some(mesh) = self.mesh.vbo_map.get(&VboKind::Vertex) {
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

            pos.x += (ch.advance >> 6) as f32 * scale;
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
