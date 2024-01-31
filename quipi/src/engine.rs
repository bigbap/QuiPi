use sdl2::EventPump;

use crate::{
    systems::{self, rendering::text::{DEFAULT_FONT, TextRenderer}},
    wrappers::sdl2::{window::QuiPiWindow, events::EventQueue}, core::time::Timer
};

#[derive(Debug)]
pub enum Flags {
    HideMouse,
    RelativeMouseMode,
}

pub trait QuiPiApp {
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
        frame_state: FrameState
    ) -> Result<bool, Box<dyn std::error::Error>>;
}

pub fn run<G: QuiPiApp>(
    game: &mut G,
    title: &str,
    width: u32,
    height: u32,
    flags: Vec<Flags>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut window_api = QuiPiWindow::init()?;
    let window = window_api.opengl_window(
        title,
        width,
        height,
        (4, 5)
    )?;

    systems::rendering::init(
        &window_api,
        width as i32,
        height as i32,
    )?;

    for flag in flags.iter() {
        match flag {
            Flags::HideMouse => window_api.ctx.mouse().show_cursor(false),
            Flags::RelativeMouseMode => window_api.ctx.mouse().set_relative_mouse_mode(true),
        }
    }
    
    game.init()?;


    let mut text = TextRenderer::new(DEFAULT_FONT)?;
    text.color = glm::vec3(1.0, 1.0, 1.0);
    text.scale = 0.7;

    let mut timer = Timer::new()?;
    let mut last_frame = timer.ticks();
    'running: loop {
        let ticks = timer.ticks();

        if !cfg!(debug_assertions) {
            // limit fps to 60
            if ticks - last_frame < 1000 / 60 {
                continue;
            }
        }

        last_frame = ticks;

        let delta = timer.delta();

        if game.handle_frame(
            FrameState {
                event_pump: &mut window_api.ctx.event_pump()?,
                // event_queue: &mut window_api.get_event_queue()?,
                text_render: &text,
                quit: false,
                delta: delta / 1000.0
            }
        )? {
            break 'running;
        }

        if cfg!(debug_assertions) {
            text.draw(
                format!("ms: {}", delta),
                glm::vec2(25.0, 50.0)
            );
            text.draw(
                format!("fps: {}", 1000.0 / delta),
                glm::vec2(25.0, 25.0)
            );
        }


        window.gl_swap_window();
    }

    Ok(())
}

pub struct FrameState<'a> {
    pub event_pump: &'a mut EventPump,
    // pub event_queue: &'a mut EventQueue,
    pub text_render: &'a TextRenderer,
    pub quit: bool,
    pub delta: f32
}
