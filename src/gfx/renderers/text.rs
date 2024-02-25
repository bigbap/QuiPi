use crate::{
    platform::opengl::{
        buffer::{
            vertex_attribute_pointer,
            Buffer,
            BufferUsage,
            VertexArray,
            VBO
        },
        capabilities::*,
        draw::*,
        shader::ShaderProgram,
        textures::gl_use_texture_unit
    },
    prelude::{
        qp_data::{
            IRenderer,
            FrameState,
        },
        qp_assets::RFont,
        GlobalRegistry,
        qp_gfx::viewport::get_dimensions
    },
    QPResult
};

#[derive(Debug)]
pub struct TextRenderer {
    shader: ShaderProgram,
    vao: VertexArray,
    vbo: Buffer<VBO>,
}

impl TextRenderer {
    pub fn new() -> QPResult<Self> {
        let shader = ShaderProgram::from_str(
            VERT_SHADER,
            FRAG_SHADER,
        )?;
        
        let vao = VertexArray::new();
        vao.bind();

        let vbo = Buffer::<VBO>::new();
        vbo.bind();
        vbo.buffer_data::<f32>(6 * 4, None, &BufferUsage::DynamicDraw);
        vertex_attribute_pointer(0, 4, std::mem::size_of::<f32>() * 4, 0);

        Ok(Self {
            shader,
            vao,
            vbo,
        })
    }
}

impl IRenderer for TextRenderer {
    fn draw(
        &mut self,
        frame_state: &mut FrameState,
        registry: &mut GlobalRegistry
    ) -> Option<u32> {
        gl_enable(GLCapability::AlphaBlending);
        gl_blending_func(GLBlendingFactor::SrcAlpha, GLBlendingFactor::OneMinusSrcAlpha);

        let (_x, _y, width, height) = get_dimensions();
        self.shader.use_program();
        self.shader.set_mat4("projection", &glm::ortho(
            0.0,
            width as f32,
            0.0,
            height as f32,
            0.0,
            0.2
        ));
        
        gl_use_texture_unit(0);

        let mut draw_calls = 0;
        for text_obj in frame_state.text_buffer.iter_mut() {
            let Some(font) = registry.asset_manager.get::<RFont>(text_obj.style.font) else {
                #[cfg(debug_assertions)]
                {
                    let font_str = registry.strings().get_string(text_obj.style.font)?;
                    println!("font '{}' is not loaded", font_str);
                }

                continue
            };

            self.shader.set_float_4("textColor", (
                text_obj.style.color.x,
                text_obj.style.color.y,
                text_obj.style.color.z,
                text_obj.style.color.w
            ));

            self.vao.bind();

            // TODO: batch this
            // let mut vertices: Vec<f32> = vec![];
            for c in text_obj.text.chars() {
                let Some(ch) = font.characters.get(c as usize) else { continue; };
    
                let x_pos = text_obj.pos.x + ch.bearing.x * text_obj.style.scale;
                let y_pos = text_obj.pos.y - (ch.size.y - ch.bearing.y) * text_obj.style.scale;
    
                let w = ch.size.x * text_obj.style.scale;
                let h = ch.size.y * text_obj.style.scale;
    
                let vertices = vec![
                    x_pos,      y_pos + h,  0.0, 0.0,
                    x_pos,      y_pos,      0.0, 1.0,
                    x_pos + w,  y_pos,      1.0, 1.0,
                    x_pos,      y_pos + h,  0.0, 0.0,
                    x_pos + w,  y_pos,      1.0, 1.0,
                    x_pos + w,  y_pos + h,  1.0, 0.0
                ];
    
                ch.texture.use_texture(0);
                self.vbo.bind();
                self.vbo.buffer_sub_data(
                    0,
                    vertices.len(),
                    Some(&vertices)
                );
                self.vbo.unbind();
    
                gl_draw(
                    DrawBuffer::Arrays,
                    DrawMode::Triangles,
                    6
                );
                draw_calls += 1;
    
                text_obj.pos.x += (ch.advance >> 6) as f32 * text_obj.style.scale;
            }
    
            self.vao.unbind();
        }

        Some(draw_calls)
    }
}

#[derive(Debug, Clone)]
pub struct QPText {
    pub text: String,
    pub pos: glm::Vec2,
    pub style: QPTextStyle,
}

#[derive(Debug, Clone)]
pub struct QPTextStyle {
    pub font: u64,
    pub color: glm::Vec4,
    pub scale: f32
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
uniform vec4 textColor;

void main()
{    
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(text, TexCoords).r);
    color = textColor * sampled;
}  
"#;
