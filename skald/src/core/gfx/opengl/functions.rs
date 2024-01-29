pub fn gl_set_viewport_dimensions(
    x: i32,
    y: i32,
    w: i32,
    h: i32
) {
    unsafe { gl::Viewport(x, y, w, h); }
}

pub fn gl_get_viewport_dimensions() -> (i32, i32, i32, i32) {
    unsafe {
        let mut out: [i32; 4] = [0, 0, 0, 0];
        gl::GetIntegerv(gl::VIEWPORT, out.as_mut_ptr());

        out.into()
    }
}

pub fn gl_scissor(x: i32, y: i32, width: i32, height: i32) {
    unsafe {
        gl::Scissor(x, y, width, height);
    }
}
