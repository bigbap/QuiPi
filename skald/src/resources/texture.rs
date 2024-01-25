use crate::Component;

#[derive(Debug, Component)]
pub struct Texture {
    pub id: u32,
}

impl Drop for Texture {
    fn drop(&mut self) {
        crate::gfx::texture::delete_texture(self.id);
    }
}
