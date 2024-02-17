#[derive(Debug, PartialEq)]
#[repr(packed)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub color: glm::Vec4,
    pub tex_coords: glm::Vec2,
    pub tex_index: f32
}

impl Vertex {
    pub fn size() -> usize {
        std::mem::size_of::<Self>()
    }
}