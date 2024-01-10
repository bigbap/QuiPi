use crate::Component;

#[derive(Debug, Component)]
pub struct ColorComponent(
    pub f32,
    pub f32,
    pub f32,
    pub f32
);
