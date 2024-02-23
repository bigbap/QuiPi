use super::super::structs::Vertex;

pub trait IMesh {
    fn vertices(&self) -> Vec<Vertex>;
    fn indices() -> Vec<i32>;
    fn vertex_count() -> usize;
}
