use crate::{
    common::resources::{ClearColor, Window},
    platform::opengl::buffer::clear_buffers,
    prelude::{App, QPError},
    world::{RenderSchedule, UpdateSchedule},
    QPResult,
};

use super::Plugin;

pub struct MainLoopPlugin {}

impl Plugin for MainLoopPlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        Ok(app.set_runner(move |mut app: App| loop {
            match app.world.execute_schedule::<UpdateSchedule>() {
                Err(QPError::Quit) => return Ok(()),
                Err(e) => return Err(e),
                _ => (),
            }

            let clr = app
                .world
                .resources
                .get::<ClearColor>()
                .unwrap_or(&ClearColor(0.3, 0.3, 0.3, 1.0));

            clear_buffers(clr.as_tuple());

            app.world.execute_schedule::<RenderSchedule>()?;

            let window = app
                .world
                .resources
                .get::<Window>()
                .ok_or(QPError::ResourceNotFound("Window".into()))?;

            if let Some(window) = &window.winapi.window {
                window.gl_swap_window();
            } else {
                return Err(QPError::ProblemSwappingFrameBuffers);
            }
        }))
    }
}
