use sdl2::video::GLProfile;

pub trait Game {
    /// game.init() is called by the engine, after all the Sdl and
    /// openGl setup is done.
    /// 
    /// Use this method to set up your game. If you do anything
    /// that uses the 'gl::' crate before this method gets called
    /// by the engine, you will get a 'function not loaded error'
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// This method is called by the engine every frame.
    /// This is where you will do all your game specific logic.
    fn handle_frame(
        &mut self,
        event_pump: &mut sdl2::EventPump
    ) -> Option<()>;
}

pub fn run<G: Game>(
    game: &mut G,
    title: &str,
    width: u32,
    height: u32
) -> Result<(), Box<dyn std::error::Error>> {
    let sdl_ctx = sdl2::init()?;
    let video_subsystem = sdl_ctx.video()?;

    sdl_ctx.mouse().show_cursor(false);
    sdl_ctx.mouse().set_relative_mouse_mode(true);

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 5);
    
    #[cfg(debug_assertions)]
    gl_attr.set_context_flags().debug().set();

    let window = video_subsystem.window(title, width, height)
        .opengl()
        .resizable()
        .build()?;

    let _gl_ctx = window.gl_create_context()?;
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (4, 5));
    
    game.init()?;

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Viewport(0, 0, width as i32, height as i32);
    }

    let mut event_pump = sdl_ctx.event_pump()?;
    'running: loop {
        if game.handle_frame(
            &mut event_pump
        ).is_none() {
            break 'running
        }

        window.gl_swap_window();
    }

    Ok(())
}
