use crate::prelude::QPError;
use crate::QPResult;
use sdl2::{
    event::Event,
    video::{GLContext, GLProfile, Window},
    Sdl, VideoSubsystem,
};

pub struct QPWindow {
    pub ctx: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub window: Option<Window>,
    pub gl_ctx: Option<GLContext>,
}

impl QPWindow {
    pub fn init() -> QPResult<Self> {
        let sdl_ctx = sdl2::init().map_err(|e| QPError::Generic(e.to_string()))?;
        let video_subsystem = sdl_ctx
            .video()
            .map_err(|e| QPError::Generic(e.to_string()))?;

        Ok(Self {
            ctx: sdl_ctx,
            video_subsystem,
            window: None,
            gl_ctx: None,
        })
    }

    pub fn opengl_window(
        &mut self,
        title: &str,
        width: u32,
        height: u32,
        gl_version: (u8, u8),
    ) -> QPResult<()> {
        let gl_attr = self.video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(gl_version.0, gl_version.1);

        #[cfg(debug_assertions)]
        gl_attr.set_context_flags().debug().set();

        let window = self
            .video_subsystem
            .window(title, width, height)
            .opengl()
            .resizable()
            .build()
            .map_err(|e| QPError::Generic(e.to_string()))?;

        self.gl_ctx = Some(
            window
                .gl_create_context()
                .map_err(|e| QPError::Generic(e.to_string()))?,
        );

        debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        debug_assert_eq!(gl_attr.context_version(), (gl_version.0, gl_version.1));

        self.window = Some(window);

        Ok(())
    }

    pub fn get_event_queue(&self) -> QPResult<Vec<Event>> {
        let mut events: Vec<Event> = vec![];

        for event in self
            .ctx
            .event_pump()
            .map_err(|e| QPError::Generic(e.to_string()))?
            .poll_iter()
        {
            events.push(event);
        }

        Ok(events)
    }

    pub fn relative_mouse_mode(&self, on: bool) {
        self.ctx.mouse().set_relative_mouse_mode(on)
    }
    pub fn get_relative_mouse_mode(&self) -> bool {
        self.ctx.mouse().relative_mouse_mode()
    }
}
