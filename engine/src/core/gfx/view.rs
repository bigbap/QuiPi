pub fn adjust_viewport_dims(w: i32, h: i32) {
    unsafe { gl::Viewport(0, 0, w, h); }
}
