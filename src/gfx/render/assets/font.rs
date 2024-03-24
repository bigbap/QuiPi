use crate::assets::{Asset, AssetLoader};
use crate::core::prelude::to_abs_path;
use crate::platform::opengl::textures::{GlTexture, ParameterName, ParameterValue};
use crate::platform::opengl::{
    pixel_store,
    textures::{Format, Target},
};
use crate::QPResult;
use ft::{face::LoadFlag, Face};

const CHARACTER_COUNT: usize = 128;

#[derive(Debug, Asset, PartialEq, Default)]
pub struct Font {
    pub characters: Vec<Character>,
}

pub struct FontLoader<'a> {
    pub path: &'a str,
}

impl<'a> AssetLoader for FontLoader<'a> {
    type AssetType = Font;

    fn load(&mut self) -> QPResult<Self::AssetType> {
        let font = to_abs_path(&format!("assets/fonts/{}.ttf", self.path))?;
        let library = ft::Library::init()?;
        let face = library.new_face(font, 0)?;

        pixel_store::set_unpack_alignment(1);

        let mut characters = Vec::<Character>::with_capacity(CHARACTER_COUNT);

        for c in 0..CHARACTER_COUNT {
            face.set_char_size(40 * 64, 0, 96, 0)?;

            if let Err(_e) = face.load_char(c, LoadFlag::RENDER) {
                #[cfg(debug_assertions)]
                println!("{}", _e);

                continue;
            }

            let width = face.glyph().bitmap().width();
            let rows = face.glyph().bitmap().rows();
            let left = face.glyph().bitmap_left();
            let top = face.glyph().bitmap_top();

            let texture = texture_from_font(&face, width, rows);

            let m_char = Character {
                texture,
                size: glm::vec2(width as f32, rows as f32),
                bearing: glm::vec2(left as f32, top as f32),
                advance_x: face.glyph().advance().x,
                advance_y: face.glyph().advance().y,
            };

            if char::from_u32(c as u32).is_some() {
                characters.push(m_char);
            }
        }

        Ok(Font { characters })
    }
}

#[derive(Debug, PartialEq)]
pub struct Character {
    pub texture: GlTexture,
    pub size: glm::Vec2,
    pub bearing: glm::Vec2,
    pub advance_x: i32,
    pub advance_y: i32,
}

// helpers

fn texture_from_font(face: &Face, width: i32, height: i32) -> GlTexture {
    let texture = GlTexture::new(width, height, Target::Texture2D);

    texture
        .bind()
        .set_parameter(ParameterName::WrapS, ParameterValue::ClampToEdge)
        .set_parameter(ParameterName::WrapT, ParameterValue::ClampToEdge)
        .set_parameter(ParameterName::MinFilter, ParameterValue::Linear)
        .set_parameter(ParameterName::MagFilter, ParameterValue::Nearest);

    texture
        .bind()
        .add_image_data(Format::Red, Format::Red, face.glyph().bitmap().buffer());

    texture
}
