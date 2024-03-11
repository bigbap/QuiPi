use std::{any::TypeId, collections::HashMap};

use crate::{
    common::resources::Camera, platform::opengl::functions::gl_set_viewport_dimensions,
    resources::*,
};

// #[derive(Resource)]
// pub struct ViewportManager {
//     viewports: HashMap<TypeId, Box<dyn ViewportTrait>>,
// }

// pub struct Dimensions {
//     pub x: i32,
//     pub y: i32,
//     pub width: i32,
//     pub height: i32,
// }

// pub trait ViewportTrait {
//     fn set_dimensions(&mut self, dims: Dimensions);

//     fn get_dimensions(&mut self) -> Dimensions;

//     fn insert_camera(&mut self, camera: Box<dyn Camera>);

//     fn get_camera(&self) -> Box<&dyn Camera>;

//     fn get_camera_mut(&mut self) -> Box<&mut dyn Camera>;
// }

#[derive(Resource)]
pub struct Viewport {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Viewport {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        let mut viewport = Self {
            x,
            y,
            width,
            height,
        };

        viewport.set_dimensions(x, y, width, height);

        viewport
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
