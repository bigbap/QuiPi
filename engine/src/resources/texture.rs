use crate::Component;

#[derive(Debug, Clone, Copy)]
pub enum TextureType {
    Diffuse,
    Specular
}

#[derive(Debug, Component)]
pub struct Texture {
    pub index: u32,
    pub kind: TextureType
}

impl Drop for Texture {
    fn drop(&mut self) {
        crate::gfx::texture::delete_texture(self.index);
    }
}
