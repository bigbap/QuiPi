use crate::{resource_manager::resources::window::Window, QPResult};

use super::Plugin;

pub struct WindowPlugin {}

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        let title = &app.config.title;
        let width = app.config.width;
        let height = app.config.height;

        let window = Window::new(title, width, height)?;

        app.add_resource(window);

        Ok(())
    }
}
