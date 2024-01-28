#[derive(Debug, Clone, Copy)]
pub enum DrawBuffer {
    Elements,
    Arrays
}

#[derive(Debug, Clone, Copy)]
pub enum DrawMode {
    Triangles,
    Lines,
    Points
}
