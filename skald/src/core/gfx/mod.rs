pub mod canvas;
pub mod image;
pub mod shader_program;
pub mod texture;
pub mod draw;
pub mod mesh;

pub use shader_program::ShaderProgram;
pub use mesh::ElementArrayMesh;
pub use draw::draw_buffer;
pub use draw::clear_buffer;

pub use opengl::capabilities as gl_capabilities;
pub use opengl::pixel_store as gl_pixel_store;

mod opengl;

use sdl2::VideoSubsystem;

use self::opengl::MyOpenGL;

pub fn init(
    video_subsystem: &VideoSubsystem,
    width: i32,
    height: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let _api = MyOpenGL::init(video_subsystem)?;

    canvas::set_dimensions(canvas::Canvas {
        x: 0,
        y: 0,
        width,
        height
    });

    Ok(())
}
