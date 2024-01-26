use crate::Component;

#[derive(Debug, Component, Default)]
pub struct CBoundingBox {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32,
    pub far: f32,
}

impl CBoundingBox {
    pub fn width(&self) -> f32 {
        (self.right - self.left).abs()
    }
    pub fn height(&self) -> f32 {
        (self.bottom - self.top).abs()
    }
}
