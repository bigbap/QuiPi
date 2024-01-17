pub mod shader_program;
pub mod texture;
pub mod object_loader;
pub mod buffer;
pub mod draw;
pub mod view;
pub mod mesh;
pub mod utils;

pub use shader_program::ShaderProgram;
pub use mesh::ElementArrayMesh;

use sdl2::VideoSubsystem;

pub fn init(
    video_subsystem: &VideoSubsystem,
    width: i32,
    height: i32
) {
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Viewport(0, 0, width, height);
    }
}
