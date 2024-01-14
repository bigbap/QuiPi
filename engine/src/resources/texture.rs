use crate::Component;

#[derive(Debug, Clone, Copy)]
pub enum TextureType {
    Diffuse,
    Specular
}

#[derive(Debug, Component)]
pub struct Texture {
    pub id: u32,
    pub index: i32, // index on the GPU
    pub kind: TextureType
}

impl Drop for Texture {
    fn drop(&mut self) {
        crate::gfx::texture::delete_texture(self.id);
    }
}
