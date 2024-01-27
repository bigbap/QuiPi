// use std::collections::HashMap;
//
// mod characters;
//
// pub struct TextRenderer {
//     library: ft::Library,
//
//     characters: HashMap<char, characters::Character>,
//     style: Style,
//     face: ft::Face
// }
//
// impl TextRenderer {
//     pub fn new(style: Style) -> Result<Self, Box<dyn std::error::Error>> {
//         Ok(Self {
//             library: ft::Library::init()?,
//             style
//         })
//     }
// }
//
// pub struct Style {
//     pub font: Fonts,
//     pub color: (f32, f32, f32, f32),
//     pub size: u16,
// }
//
// impl Default for Style {
//     fn default() -> Self {
//         Self {
//             font: Fonts::OpenSansRegular("assets/fonts/"),
//             color: (1.0, 1.0, 1.0, 1.0),
//             size: 24
//         }
//     }
// }
//
// pub enum Fonts {
//     OpenSansRegular(String)
// }
//
// fn load_face(
//     font: Fonts,
//     library: &mut ft::Library
// ) -> Result<ft::Face, Box<dyn std::error::Error>> {
//     Ok(match font {
//         Fonts::OpenSansRegular => library.new_face(path, 0)?
//     })
// }
