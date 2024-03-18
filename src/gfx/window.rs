use crate::{
    platform::sdl2::QPWindow,
    plugin::Plugin,
    prelude::{qp_gfx::Viewport, QPError},
    resources::{AsAny, Resource},
    QPResult,
};

use super::prelude::init;

pub struct WindowPlugin {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        let window = Window::new(&self.title, self.width, self.height)?;
        let viewport = Viewport::new(0, 0, self.width as i32, self.height as i32);

        app.add_resource(window);
        app.add_resource(viewport);

        Ok(())
    }
}

#[derive(Resource, AsAny)]
pub struct Window {
    pub winapi: QPWindow,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> QPResult<Self> {
        let mut winapi = QPWindow::init().unwrap();
        let _window = winapi.opengl_window(title, width, height, (4, 5))?;

        init(&winapi).map_err(|e| QPError::Generic(e.to_string()))?;

        Ok(Self { winapi })
    }
}
