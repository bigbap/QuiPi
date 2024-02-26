use crate::core::prelude::to_abs_path;
use crate::prelude::qp_data::{IMesh, Vertex};
use crate::prelude::qp_ecs::Component;
use crate::platform::opengl::{
    pixel_store,
    textures::{
        Texture,
        Target,
        Format,
    }
};
use crate::QPResult;
use ft::{
    face::LoadFlag,
    Face,
};

const CHARACTER_COUNT: usize = 128;

#[derive(Debug, Component, PartialEq)]
pub struct RFont {
    // pub texture: Texture,
    pub characters: Vec<QPCharacter>
}

impl RFont {
    pub fn new(font: &str) -> QPResult<RFont> {
        let font = to_abs_path(&format!("assets/fonts/{font}.ttf"))?;
        let library = ft::Library::init()?;
        let face = library.new_face(font, 0)?;

        pixel_store::set_unpack_alignment(1);

        let mut characters = Vec::<QPCharacter>::with_capacity(CHARACTER_COUNT);

        for c in 0..CHARACTER_COUNT {
            face.set_char_size(40 * 64, 0, 96, 0)?;

            if let Err(_e) = face.load_char(c, LoadFlag::RENDER) {
                #[cfg(debug_assertions)]
                println!("{}", _e);
    
                continue
            }

            let width = face.glyph().bitmap().width();
            let rows = face.glyph().bitmap().rows();
            let left = face.glyph().bitmap_left();
            let top = face.glyph().bitmap_top();

            let texture = texture_from_font(
                &face,
                width,
                rows
            );
    
            let m_char = QPCharacter {
                texture,
                size: glm::vec2(width as f32, rows as f32),
                bearing: glm::vec2(left as f32, top as f32),
                advance_x: face.glyph().advance().x,
                advance_y: face.glyph().advance().y,
                pos: glm::vec2(0.0, 0.0),
                scale: 1.0
            };
    
            if char::from_u32(c as u32).is_some() {
                characters.push(m_char);
            }
        }

        Ok(Self {
            characters
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct QPCharacter {
    pub texture: Texture,
    pub size: glm::Vec2,
    pub bearing: glm::Vec2,
    pub advance_x: i32,
    pub advance_y: i32,

    // used to determine the vertices
    pub pos: glm::Vec2,
    pub scale: f32
}

impl IMesh for QPCharacter {
    fn indices() -> Vec<i32> { vec![0, 1, 2, 0, 2, 3] }
    fn vertex_count() -> usize { 6 }

    fn vertices(&self) -> Vec<Vertex> {
        let x_pos = self.pos.x + self.bearing.x * self.scale;
        let y_pos = self.pos.y - (self.size.y - self.bearing.y) * self.scale;

        let w = self.size.x * self.scale;
        let h = self.size.y * self.scale;

        vec![
            Vertex {
                position: glm::vec3(x_pos, y_pos + h, 0.0),
                color: glm::vec4(1.0, 1.0, 1.0, 1.0),
                tex_coords: glm::vec2(0.0, 0.0),
                tex_index: 0.0
            },
            Vertex {
                position: glm::vec3(x_pos, y_pos, 0.0),
                color: glm::vec4(1.0, 1.0, 1.0, 1.0),
                tex_coords: glm::vec2(0.0, 1.0),
                tex_index: 0.0
            },
            Vertex {
                position: glm::vec3(x_pos + w, y_pos, 0.0),
                color: glm::vec4(1.0, 1.0, 1.0, 1.0),
                tex_coords: glm::vec2(1.0, 1.0),
                tex_index: 0.0
            },
            Vertex {
                position: glm::vec3(x_pos + w, y_pos + h, 0.0),
                color: glm::vec4(1.0, 1.0, 1.0, 1.0),
                tex_coords: glm::vec2(1.0, 0.0),
                tex_index: 0.0
            },
        ]
    }
}

// helpers

fn texture_from_font(
    face: &Face,
    width: i32,
    height: i32
) -> Texture {
    let texture = Texture::new(
        width,
        height,
        Target::Texture2D
    );

    texture
        .bind()
        .add_image_data(
            Format::Red,
            Format::Red,
            face
                .glyph()
                .bitmap()
                .buffer(),
        );

    texture
}