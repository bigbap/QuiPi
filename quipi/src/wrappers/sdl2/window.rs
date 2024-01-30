use sdl2::{
    video::{
        Window,
        GLProfile, GLContext,
    },
    VideoSubsystem,
    Sdl
};

pub struct QuiPiWindow {
    pub ctx: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub gl_ctx: Option<GLContext>
}

impl QuiPiWindow {
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let sdl_ctx = sdl2::init()?;
        let video_subsystem = sdl_ctx.video()?;

        Ok(Self {
            ctx: sdl_ctx,
            video_subsystem,
            gl_ctx: None
        })
    }

    pub fn opengl_window(
        &mut self,
        title: &str,
        width: u32,
        height: u32,
        gl_version: (u8, u8)
    ) -> Result<Window, Box<dyn std::error::Error>> {
        let gl_attr = self.video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(gl_version.0, gl_version.1);

        #[cfg(debug_assertions)]
        gl_attr.set_context_flags().debug().set();

        let window = self.video_subsystem.window(title, width, height)
            .opengl()
            .resizable()
            .build()?;

        self.gl_ctx = Some(window.gl_create_context()?);

        debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        debug_assert_eq!(gl_attr.context_version(), (gl_version.0, gl_version.1));

        Ok(window)
    }
}

