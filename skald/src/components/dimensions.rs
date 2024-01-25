use crate::Component;

#[derive(Debug, Component, Default)]
pub struct CDimensions {
    pub width: f32,
    pub height: f32,
    pub depth: f32
}
