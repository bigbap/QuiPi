pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub tex_coords: [f32; 2],
    pub tex_index: f32
}

impl Vertex {
    pub fn size() -> usize {
        std::mem::size_of::<Self>()
    }
}