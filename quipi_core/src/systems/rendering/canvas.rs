use crate::wrappers::opengl;

pub fn set_dimensions(
    x: i32,
    y: i32,
    width: i32,
    height: i32
) {
    opengl::functions::gl_set_viewport_dimensions(
        x,
        y,
        width,
        height
    );
}

/**
* returns (x, y, width, height)  
*/
pub fn get_dimensions() -> (i32, i32, i32, i32) {
    let dims = opengl::functions::gl_get_viewport_dimensions();

    (dims.0, dims.1, dims.2, dims.3)
}
