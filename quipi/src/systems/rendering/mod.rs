use crate::{
    facades::opengl::MyOpenGL,
    Registry,
    VersionedIndex
};

pub mod renderer;
pub mod renderer_2d;
pub mod canvas;
pub mod mesh;
pub mod texture;

pub use renderer::Renderer;
pub use renderer_2d::Renderer2D;
use sdl2::VideoSubsystem;

mod draw;
mod matrices;

pub trait IRenderer {
    fn camera(&self) -> VersionedIndex;
    fn update_view_matrix(&self, registry: &mut Registry);
}

pub fn init(
    video_subsystem: &VideoSubsystem,
    width: i32,
    height: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let _opengl = MyOpenGL::init(video_subsystem)?;

    canvas::set_dimensions(0, 0, width, height);

    Ok(())
}