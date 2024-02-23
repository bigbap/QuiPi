#[derive(Debug, PartialEq, Clone)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub color: glm::Vec4,
    pub tex_coords: glm::Vec2,
    pub tex_index: f32
}