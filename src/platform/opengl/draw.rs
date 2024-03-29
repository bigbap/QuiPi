use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum DrawBuffer {
    Elements,
    Arrays
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DrawMode {
    Triangles,
    Lines,
    Points
}

pub fn gl_draw(
    kind: DrawBuffer,
    mode: DrawMode,
    count: i32
) {
    match kind {
        DrawBuffer::Elements => draw_elements(count, mode),
        DrawBuffer::Arrays => draw_arrays(count, mode)
    }
}

fn draw_elements(count: i32, mode: DrawMode) {
    unsafe {
        gl::DrawElements(
            match mode {
                DrawMode::Triangles => gl::TRIANGLES,
                DrawMode::Lines => gl::LINES,
                DrawMode::Points => gl::POINTS
            },
            count,
            gl::UNSIGNED_INT,
            std::ptr::null()
        );
    }
}

fn draw_arrays(count: i32, mode: DrawMode) {
    unsafe {
        gl::DrawArrays(
            match mode {
                DrawMode::Triangles => gl::TRIANGLES,
                DrawMode::Lines => gl::LINES,
                DrawMode::Points => gl::POINTS
            },
            0,
            count
        );
    }
}
