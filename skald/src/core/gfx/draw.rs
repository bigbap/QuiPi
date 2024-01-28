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

pub fn draw_buffer(
    kind: DrawBuffer,
    mode: DrawMode,
    count: i32
) {
    opengl::draw::draw(kind, mode, count);
}

pub fn clear_buffer(clr: (f32, f32, f32, f32)) {
    opengl::buffer::clear_buffers(clr);
}

pub fn define_scissor_rect(x: i32, y: i32, width: i32, height: i32) {
    opengl::functions::scissor(x, y, width, height);
}
