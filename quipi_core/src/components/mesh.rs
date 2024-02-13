use crate::Component;

#[derive(Debug, Component, PartialEq, Default)]
pub struct CMeshData {
    pub indices: Vec<u32>,
    pub vertices: Vec<f32>,
    pub colors: Vec<f32>,
    pub normals: Vec<f32>,
    pub tex_coords: Vec<f32>,
}
