use crate::{common::resources::window::Window, QPResult};

use super::Plugin;

pub struct WindowPlugin {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        let window = Window::new(&self.title, self.width, self.height)?;

        app.add_resource(window);

        Ok(())
    }
}
