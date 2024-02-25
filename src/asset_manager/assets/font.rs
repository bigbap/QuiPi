use crate::core::prelude::to_abs_path;
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
    pub characters: Vec<Character>
}

impl RFont {
    pub fn new(font: &str) -> QPResult<RFont> {
        let font = to_abs_path(&format!("assets/fonts/{font}.ttf"))?;
        let library = ft::Library::init()?;
        let face = library.new_face(font, 0)?;

        pixel_store::set_unpack_alignment(1);

        let mut characters = Vec::<Character>::with_capacity(CHARACTER_COUNT);

        for c in 0..CHARACTER_COUNT {
            face.set_char_size(40 * 64, 0, 50, 0)?;

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
    
            let m_char = Character {
                texture,
                size: glm::vec2(width as f32, rows as f32),
                bearing: glm::vec2(left as f32, top as f32),
                advance: face.glyph().advance().x
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
pub struct Character {
    pub texture: Texture,
    pub size: glm::Vec2,
    pub bearing: glm::Vec2,
    pub advance: i32
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