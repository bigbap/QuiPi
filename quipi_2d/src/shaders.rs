pub static SPRITE_VERT: &str = include_str!("shaders/sprite.vert");
pub static SPRITE_FRAG: &str = include_str!("shaders/sprite.frag");

pub fn get_shader(shader: &str) -> ShaderResult {
    match shader {
        "sprite" => ShaderResult { vert: SPRITE_VERT, frag: SPRITE_FRAG },
        _ => ShaderResult { vert: SPRITE_VERT, frag: SPRITE_FRAG } // default
    }
}

pub struct ShaderResult {
    pub vert: &'static str,
    pub frag: &'static str
}