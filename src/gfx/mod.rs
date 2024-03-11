// mod grid;
mod batch;
// mod renderers;
// mod shaders_delete;
mod texture;
mod viewport;

pub mod prelude {
    use crate::{
        platform::{opengl::MyOpenGL, sdl2::QPWindow},
        QPResult,
    };

    use super::*;

    // pub use grid::*;
    pub use batch::*;
    // pub use renderers::*;
    // pub use shaders_delete::*;
    pub use texture::texture;
    pub use viewport::Viewport;

    pub fn init(window_api: &QPWindow) -> QPResult<()> {
        let _opengl = MyOpenGL::init(window_api)?;

        Ok(())
    }
}
