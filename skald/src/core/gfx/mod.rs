pub mod canvas;
pub mod image;
pub mod shader_program;
pub mod texture;
pub mod draw;
pub mod mesh;
pub mod opengl;

pub use shader_program::ShaderProgram;
pub use mesh::ElementArrayMesh;
// pub use draw::draw_buffer;
// pub use draw::clear_buffer;

use sdl2::VideoSubsystem;

use self::opengl::MyOpenGL;

pub fn init(
    video_subsystem: &VideoSubsystem,
    width: i32,
    height: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let _api = MyOpenGL::init(video_subsystem)?;

    canvas::set_dimensions(0, 0, width, height);

    Ok(())
}
