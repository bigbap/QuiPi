// mod grid;
pub mod render;
mod shaders;
mod sprite;
pub mod window;

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
    pub use window::*;

    pub fn init(window_api: &QPWindow) -> QPResult<()> {
        let _opengl = MyOpenGL::init(window_api)?;

        Ok(())
    }
}
