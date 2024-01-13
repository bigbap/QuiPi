use crate::gfx::buffer;

pub fn draw_ebo(vao: &buffer::VertexArray) {
    unsafe {
        gl::DrawElements(
            gl::TRIANGLES,
            vao.count(),
            gl::UNSIGNED_INT,
            std::ptr::null()
        );
    }
}
