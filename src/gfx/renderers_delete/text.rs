use crate::{
    gfx::batch_renderer::Vertex,
    platform::opengl::capabilities::*,
    prelude::{
        qp_assets::{RFont, RShader},
        qp_gfx::BatchRenderer,
        Renderer, World,
    },
    QPResult,
};

pub struct TextRenderer {
    shader: RShader,

    renderer: BatchRenderer<10000, 4>,
}

impl TextRenderer {
    pub fn new() -> QPResult<Self> {
        let shader = RShader::from_str(VERT_SHADER, FRAG_SHADER, vec![])?;

        Ok(Self {
            shader,
            renderer: BatchRenderer::new(vec![0, 1, 2, 0, 2, 3]),
        })
    }
}

impl Renderer for TextRenderer {
    fn draw(&mut self, world: &mut World) -> Option<u32> {
        gl_enable(GLCapability::AlphaBlending);
        gl_blending_func(
            GLBlendingFactor::SrcAlpha,
            GLBlendingFactor::OneMinusSrcAlpha,
        );

        let (_x, _y, width, height) = world.viewport.get_dimensions();

        let projection = &glm::ortho(0.0, width as f32, 0.0, height as f32, 0.0, 0.2);

        self.renderer.reset_info();
        self.renderer.begin_batch();
        for text_obj in world.text_buffer.iter_mut() {
            let Some(font) = world
                .registry
                .asset_manager
                .get::<RFont>(text_obj.style.font)
            else {
                #[cfg(debug_assertions)]
                {
                    println!("font is not loaded");
                }

                continue;
            };

            for c in text_obj.text.chars() {
                let Some(ch) = font.characters.get(c as usize) else {
                    continue;
                };

                let x_pos = text_obj.pos.x + ch.bearing.x * text_obj.style.scale;
                let y_pos = text_obj.pos.y - (ch.size.y - ch.bearing.y) * text_obj.style.scale;

                let w = ch.size.x * text_obj.style.scale;
                let h = ch.size.y * text_obj.style.scale;

                let mesh = CharacterMesh {
                    pos: glm::vec4(x_pos, y_pos, 0.0, 1.0),
                    projection: *projection,
                    color: text_obj.style.color,
                    w,
                    h,
                };

                self.renderer
                    .draw(mesh.vertices(), &self.shader, Some(&ch.texture));

                text_obj.pos.x += (ch.advance_x >> 6) as f32 * text_obj.style.scale;
            }
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
    pub scale: f32,
}

struct CharacterMesh {
    pos: glm::Vec4,
    projection: glm::Mat4,
    color: glm::Vec4,
    w: f32,
    h: f32,
}

impl CharacterMesh {
    fn vertices(&self) -> [Vertex; 4] {
        let pos1 = self.projection * glm::vec4(self.pos.x, self.pos.y + self.h, 0.0, 1.0);
        let pos2 = self.projection * glm::vec4(self.pos.x, self.pos.y, 0.0, 1.0);
        let pos3 = self.projection * glm::vec4(self.pos.x + self.w, self.pos.y, 0.0, 1.0);
        let pos4 = self.projection * glm::vec4(self.pos.x + self.w, self.pos.y + self.h, 0.0, 1.0);

        [
            Vertex {
                position: pos1.xyz(),
                color: self.color,
                tex_coords: glm::vec2(0.0, 0.0),
                tex_index: 0.0,
            },
            Vertex {
                position: pos2.xyz(),
                color: self.color,
                tex_coords: glm::vec2(0.0, 1.0),
                tex_index: 0.0,
            },
            Vertex {
                position: pos3.xyz(),
                color: self.color,
                tex_coords: glm::vec2(1.0, 1.0),
                tex_index: 0.0,
            },
            Vertex {
                position: pos4.xyz(),
                color: self.color,
                tex_coords: glm::vec2(1.0, 0.0),
                tex_index: 0.0,
            },
        ]
    }
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

uniform sampler2D u_textures[32];

out vec4 fragColor;

void main()
{
    int texId = int(texIndex);
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(u_textures[texId], texCoords).r);
    fragColor = color * sampled;
}
"#;
