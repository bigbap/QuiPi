use crate::gfx::buffer;

#[derive(Debug, Clone, Copy)]
pub enum DrawMode {
    Triangles,
    Lines,
    Points
}

pub fn draw_ebo(
    vao: &buffer::VertexArray,
    mode: DrawMode
) {
    unsafe {
        gl::DrawElements(
            match mode {
                DrawMode::Triangles => gl::TRIANGLES,
                DrawMode::Lines => gl::LINES,
                DrawMode::Points => gl::POINTS
            },
            vao.count(),
            gl::UNSIGNED_INT,
            std::ptr::null()
        );
    }
}
