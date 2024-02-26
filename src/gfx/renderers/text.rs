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
        qp_assets::{QPCharacter, RFont, RShader}, qp_data::{
            FrameState, IRenderer
        }, qp_gfx::{viewport::get_dimensions, BatchRenderer}, GlobalRegistry
    },
    QPResult
};

pub struct TextRenderer {
    shader: RShader,
    // vao: VertexArray,
    // vbo: Buffer<VBO>,

    renderer: BatchRenderer<10000, QPCharacter>
}

impl TextRenderer {
    pub fn new() -> QPResult<Self> {
        let shader = RShader::from_str(
            VERT_SHADER,
            FRAG_SHADER,
            vec![]
        )?;
        
        // let vao = VertexArray::new();
        // vao.bind();

        // let vbo = Buffer::<VBO>::new();
        // vbo.bind();
        // vbo.buffer_data::<f32>(6 * 4, None, &BufferUsage::DynamicDraw);
        // vertex_attribute_pointer(0, 4, std::mem::size_of::<f32>() * 4, 0);
        // vertex_attribute_pointer(1, 4, std::mem::size_of::<f32>() * 4, 0);

        Ok(Self {
            shader,
            // vao,
            // vbo,

            renderer: BatchRenderer::new()
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

        let projection = &glm::ortho(
            0.0,
            frame_state.viewport.width as f32,
            0.0,
            frame_state.viewport.height as f32,
            0.0,
            0.2
        );

        // self.shader.use_program();
        // self.shader.set_mat4("projection", );
        
        gl_use_texture_unit(0);

        self.renderer.reset_info();
        self.renderer.begin_batch();
        for text_obj in frame_state.text_buffer.iter_mut() {
            let Some(font) = registry.asset_manager.get::<RFont>(text_obj.style.font) else {
                #[cfg(debug_assertions)]
                {
                    let font_str = registry.strings().get_string(text_obj.style.font)?;
                    println!("font '{}' is not loaded", font_str);
                }

                continue
            };

            // self.shader.set_float_4("textColor", (
            //     text_obj.style.color.x,
            //     text_obj.style.color.y,
            //     text_obj.style.color.z,
            //     text_obj.style.color.w
            // ));

            // self.vao.bind();

            // TODO: batch this
            // let mut vertices: Vec<f32> = vec![];
            for c in text_obj.text.chars() {
                let Some(ch) = font.characters.get(c as usize) else { continue; };
    
                let x_pos = text_obj.pos.x + ch.bearing.x * text_obj.style.scale;
                let y_pos = text_obj.pos.y - (ch.size.y - ch.bearing.y) * text_obj.style.scale;
    
                let w = ch.size.x * text_obj.style.scale;
                let h = ch.size.y * text_obj.style.scale;
    
                let vertices = vec![
                    x_pos,      y_pos + h,  0.0, 0.0, // 0
                    x_pos,      y_pos,      0.0, 1.0, // 1
                    x_pos + w,  y_pos,      1.0, 1.0, // 2
                    x_pos,      y_pos + h,  0.0, 0.0, // 0
                    x_pos + w,  y_pos,      1.0, 1.0, // 2
                    x_pos + w,  y_pos + h,  1.0, 0.0, // 3
                ];
    
                // ch.texture.use_texture(0);
                // self.vbo.bind();
                // self.vbo.buffer_sub_data(
                //     0,
                //     vertices.len(),
                //     Some(&vertices)
                // );
                // self.vbo.unbind();
    
                // gl_draw(
                //     DrawBuffer::Arrays,
                //     DrawMode::Triangles,
                //     6
                // );
    
                text_obj.pos.x += (ch.advance_x >> 6) as f32 * text_obj.style.scale;
            }
    
            // self.vao.unbind();
        }
        self.renderer.end_batch();
        self.renderer.flush_batch(&self.shader);

        Some(self.renderer.draw_calls)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct QPText {
    pub text: String,
    pub pos: glm::Vec2,
    pub style: QPTextStyle,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct QPTextStyle {
    pub font: u64,
    pub color: glm::Vec4,
    pub scale: f32
}

const VERT_SHADER: &str = r#"
#version 450 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec2 aTexCoords;
layout (location = 3) in float aTexIndex;

out vec4 color;
out vec2 texCoords;
out float texIndex;

void main()
{
    gl_Position = vec4(aPos, 1.0);
    
    color = aColor;
    texCoords = aTexCoords;
    texIndex = aTexIndex;
}
"#;

const FRAG_SHADER: &str = r#"
#version 450 core

in vec4 color;
in vec2 texCoords;
in float texIndex;

uniform sampler2D u_textures[16];

out vec4 fragColor;

void main()
{
    int texId = int(texIndex);
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(u_textures[texId], texCoords).r);
    fragColor = color * sampled;
}
"#;
