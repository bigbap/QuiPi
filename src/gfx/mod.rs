// mod grid;
mod shaders;
mod sprite;

pub mod render;

pub mod prelude {
    use crate::{
        platform::{opengl::MyOpenGL, sdl2::QPWindow},
        QPResult,
    };

    use super::*;

    // pub use grid::*;

    pub use render::batch::*;
    pub use render::cameras::*;
    pub use render::texture::texture;
    pub use render::viewport::Viewport;
    pub use shaders::*;
    pub use sprite::*;

    pub fn init(window_api: &QPWindow) -> QPResult<()> {
        let _opengl = MyOpenGL::init(window_api)?;

        Ok(())
    }
}
