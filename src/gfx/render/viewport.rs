use crate::{platform::opengl::functions::gl_set_viewport_dimensions, resources::*};

#[derive(Resource, AsAny)]
pub struct Viewport {
    dims: ViewportDimensions,
}

impl Viewport {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        let mut viewport = Self {
            dims: ViewportDimensions {
                x,
                y,
                width,
                height,
            },
        };

        viewport.set_dimensions(viewport.dims);

        viewport
    }

    pub fn set_dimensions(&mut self, dims: ViewportDimensions) {
        self.dims = dims;

        gl_set_viewport_dimensions(dims.x, dims.y, dims.width, dims.height);
    }

    pub fn get_dimensions(&self) -> ViewportDimensions {
        self.dims
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ViewportDimensions {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
