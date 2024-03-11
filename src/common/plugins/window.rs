use crate::{common::resources::window::Window, prelude::qp_gfx::Viewport, QPResult};

use super::Plugin;

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
