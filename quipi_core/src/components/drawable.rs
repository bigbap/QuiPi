use crate::Component;

#[derive(Debug, Component, PartialEq)]
pub struct CDrawable {
    pub shader: u64,
    pub camera: u64,

    pub color: Option<glm::Vec4>,
    pub texture: Option<u64>
}
