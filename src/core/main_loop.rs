use crate::{
    platform::opengl::buffer::clear_buffers,
    plugin::Plugin,
    prelude::{qp_gfx::Window, App, QPError},
    schedule::{Cleanup, Render, Update},
    QPResult,
};

use super::clear_color::ClearColor;

pub struct MainLoopPlugin;

impl Plugin for MainLoopPlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        Ok(app.set_runner(move |mut app: App| loop {
            app.world.execute(Cleanup)?;
            app.world.execute(Update)?;

            if app.world.quitting {
                return Ok(());
            }

            let clr = app
                .world
                .resources
                .get::<ClearColor>()
                .unwrap_or(&ClearColor(0.3, 0.3, 0.3, 1.0));

            clear_buffers(clr.as_tuple());

            app.world.execute(Render)?;

            let Some(window) = &app
                .world
                .resources
                .get::<Window>()
                .ok_or(QPError::ResourceNotFound("Window".into()))?
                .winapi
                .window
            else {
                return Err(QPError::ProblemSwappingFrameBuffers);
            };

            window.gl_swap_window()
        }))
    }
}
