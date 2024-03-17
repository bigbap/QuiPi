use crate::{
    common::resources::{ClearColor, Window},
    platform::opengl::buffer::clear_buffers,
    prelude::{App, QPError},
    schedule::{Render, Update},
    QPResult,
};

use super::Plugin;

pub struct MainLoopPlugin {}

impl Plugin for MainLoopPlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        Ok(app.set_runner(move |mut app: App| loop {
            app.world.execute(Update)?;

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
