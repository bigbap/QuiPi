// mod grid;
mod batch_renderer;
mod renderers;
mod shaders;
mod texture;
mod viewport;

pub mod prelude {
    use crate::{
        platform::{
            opengl::{functions, MyOpenGL},
            sdl2::QPWindow,
        },
        QPResult,
    };

    use super::*;

    // pub use grid::*;
    pub use batch_renderer::BatchRenderer;
    pub use renderers::*;
    pub use shaders::*;
    pub use texture::texture;
    pub use viewport::Viewport;

    pub struct Gfx {
        pub viewport: Viewport,
    }

    impl Gfx {
        pub fn init(window_api: &QPWindow, width: i32, height: i32) -> QPResult<Self> {
            let _opengl = MyOpenGL::init(window_api)?;

            functions::gl_set_viewport_dimensions(0, 0, width, height);

            Ok(Self {
                viewport: Viewport::new(0, 0, width, height),
            })
        }
    }
}
