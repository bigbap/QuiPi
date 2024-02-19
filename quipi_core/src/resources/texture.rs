use crate::{
    Component,
    platform::opengl::textures::Texture
};

#[derive(Component, Debug, PartialEq)]
pub struct RTexture {
    pub texture: Texture,
    pub texture_dims: glm::Vec2
}

// impl RTexture {
//     pub fn get_texture_coords(loc: u32, texture_dims: glm::Vec2) -> glm::Vec2 {

//     }
// }
