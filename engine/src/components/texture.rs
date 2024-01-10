use crate::Component;

#[derive(Debug)]
pub enum TextureType {
    Diffuse,
    Specular
}

#[derive(Debug, Component)]
pub struct TextureComponent {
    pub id: u32,
    pub kind: TextureType
}

