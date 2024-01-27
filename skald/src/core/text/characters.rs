use std::char;
use std::collections::HashMap;
use ft::{
    face::LoadFlag,
    FtResult
};

use crate::gfx::{
    gl_pixel_store,
    texture::{
        self,
        ITexture
    }
};

pub struct Character {
    pub texture: Box<dyn ITexture>,
    pub size: glm::Vec2,
    pub bearing: glm::Vec2,
    pub advance: i32
}

pub fn load_char_textures(font: &str) -> FtResult<HashMap<char, Character>> {
    let library = ft::Library::init()?;
    let face = library.new_face(font, 0)?;

    gl_pixel_store::set_unpack_alignment(1);

    let mut map: HashMap<char, Character> = HashMap::new();

    for c in 0..128 {
        if let Err(e) = face.load_char(c, LoadFlag::RENDER) {
            #[cfg(debug_assertions)]
            println!("{}", e);

            continue
        }

        let width = face.glyph().bitmap().width();
        let rows = face.glyph().bitmap().rows();
        let left = face.glyph().bitmap_left();
        let top = face.glyph().bitmap_top();
        let texture = texture::from_font(
            &face,
            width,
            rows
        ).map_err(|e| {
            #[cfg(debug_assertions)]
            println!("{:?}", e);

            ft::Error::Unknown
        })?;

        let m_char = Character {
            texture,
            size: glm::vec2(width as f32, rows as f32),
            bearing: glm::vec2(left as f32, top as f32),
            advance: face.glyph().advance().x
        };

        if let Some(c) = char::from_u32(c as u32) {
            map.insert(c, m_char);
        }
    }

    Ok(map)
}
