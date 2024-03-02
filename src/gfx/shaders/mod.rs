use serde::{Deserialize, Serialize};

pub static SPRITE_VERT: &str = include_str!("sprite.vert");
pub static SPRITE_FRAG: &str = include_str!("sprite.frag");

pub fn get_shader(shader: &str) -> ShaderResult {
    match shader {
        "sprite" => ShaderResult {
            vert: SPRITE_VERT,
            frag: SPRITE_FRAG,
        },
        _ => ShaderResult {
            vert: SPRITE_VERT,
            frag: SPRITE_FRAG,
        }, // default
    }
}

pub struct ShaderResult {
    pub vert: &'static str,
    pub frag: &'static str,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ShaderUniforms {
    MVPMatrix(String),
    ModelMatrix(String),
    ViewMatrix(String),
    ProjectionMatrix(String),
    Color(String),
    NearPlane(String),
    FarPlane(String),
}
