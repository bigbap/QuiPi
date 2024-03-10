use crate::prelude::qp_ecs::*;

#[derive(Debug, Component, PartialEq, Default)]
pub struct CMeshData {
    pub indices: Vec<u32>,
    pub normals: Vec<f32>,
}
