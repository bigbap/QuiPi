use crate::platform::opengl::functions::gl_set_viewport_dimensions;

pub struct Viewport {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Viewport {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn set_dimensions(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;

        gl_set_viewport_dimensions(x, y, width, height);
    }

    /**
     * returns (x, y, width, height)  
     */
    pub fn get_dimensions(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, self.width, self.height)
    }
}
