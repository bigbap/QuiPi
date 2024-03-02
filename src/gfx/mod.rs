// mod grid;
mod batch_renderer;
mod renderers;
mod shaders;
mod texture;
mod viewport;

pub mod prelude {
    use crate::{
        platform::{opengl::MyOpenGL, sdl2::QPWindow},
        QPResult,
    };

    use super::*;

    // pub use grid::*;
    pub use batch_renderer::*;
    pub use renderers::*;
    pub use shaders::*;
    pub use texture::texture;
    pub use viewport::Viewport;

    pub fn init(window_api: &QPWindow) -> QPResult<()> {
        let _opengl = MyOpenGL::init(window_api)?;

        Ok(())
    }
}
