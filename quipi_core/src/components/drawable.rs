use crate::Component;

#[derive(Debug, Component, PartialEq)]
pub struct CDrawable {
    pub color: Option<glm::Vec4>,
    pub texture: Option<u64>
}
