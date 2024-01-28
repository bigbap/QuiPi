use super::opengl;

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

pub fn call_api_draw(
    kind: DrawBuffer,
    mode: DrawMode,
    count: i32
) {
    opengl::draw::draw(kind, mode, count);
}

pub fn call_api_clear(clr: (f32, f32, f32, f32)) {
    opengl::buffer::clear_buffers(clr);
}
