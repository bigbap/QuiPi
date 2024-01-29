use crate::systems::ec_store::Component;

#[derive(Debug, Component, PartialEq, Clone)]
pub struct CTarget {
    pub x: f32,
    pub y: f32,
    pub z: f32
}
