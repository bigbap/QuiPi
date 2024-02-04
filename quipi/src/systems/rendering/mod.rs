use crate::{
    wrappers::{
        opengl::MyOpenGL,
        sdl2::window::QuiPiWindow
    },
    Registry,
    VersionedIndex
};

pub mod renderer;
pub mod renderer_2d;
pub mod canvas;
pub mod mesh;
pub mod texture;
pub mod text;
pub mod draw;

// pub use renderer::Renderer;
// pub use renderer_2d::Renderer2D;

pub trait IRenderer {
    fn camera(&self) -> VersionedIndex;
    fn update_view_matrix(&self, registry: &mut Registry);
}

pub fn init(
    window_api: &QuiPiWindow,
    width: i32,
    height: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let _opengl = MyOpenGL::init(window_api)?;

    canvas::set_dimensions(0, 0, width, height);

    Ok(())
}
