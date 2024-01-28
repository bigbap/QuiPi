use super::opengl;

#[derive(Debug)]
pub struct Canvas {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32
}

pub fn set_dimensions(canvas: Canvas) {
    opengl::functions::set_viewport_dimensions(
        canvas.x,
        canvas.y,
        canvas.width,
        canvas.height
    );
}

pub fn get_dimensions() -> Canvas {
    let dims = opengl::functions::get_viewport_dimensions();

    Canvas {
        x: dims.0,
        y: dims.1,
        width: dims.2,
        height: dims.3
    }
}
