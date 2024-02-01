use crate::{
    systems::{
        self,
        rendering::text::{
            DEFAULT_FONT,
            TextRenderer
        }
    },
    wrappers::{
        sdl2::window::QuiPiWindow,
        egui::GUI
    },
    core::time::Timer
};

pub trait QuiPiApp {
    /// game.init() is called by the engine, after all the Sdl and
    /// openGl setup is done.
    /// 
    /// Use this method to set up your game. If you do anything
    /// that uses the 'gl::' crate before this method gets called
    /// by the engine, you will get a 'function not loaded error'
    fn init(&mut self, winapi: &QuiPiWindow) -> Result<(), Box<dyn std::error::Error>>;
    
    /// This method is called by the engine every frame.
    /// This is where you will do all your game specific logic.
    fn handle_frame(
        &mut self,
        frame_state: &mut AppState
    ) -> Result<FrameResponse, Box<dyn std::error::Error>>;
}

pub fn run<G: QuiPiApp>(
    game: &mut G,
    title: &str,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut winapi = QuiPiWindow::init()?;
    let window = winapi.opengl_window(
        title,
        width,
        height,
        (4, 5)
    )?;

    systems::rendering::init(
        &winapi,
        width as i32,
        height as i32,
    )?;
    
    game.init(&winapi)?;

    let mut text = TextRenderer::new(DEFAULT_FONT)?;
    let mut gui = GUI::new(1.0)?;
    let mut timer = Timer::new()?;

    let mut app_state = AppState {
        input_owner: InputOwner::App,
        winapi: &mut winapi,
        text_render: &mut text,
        delta: timer.delta()
    };

    'running: loop {
        match game.handle_frame(&mut app_state)? {
            FrameResponse::Quit => break 'running,
            FrameResponse::RelinquishInput => app_state.input_owner = InputOwner::Editor,
            FrameResponse::Ignore => ()
        }

        match gui.update(&mut app_state)? {
            FrameResponse::Quit => break 'running,
            FrameResponse::RelinquishInput => app_state.input_owner = InputOwner::App,
            FrameResponse::Ignore => ()
        }

        print_debug(&app_state, app_state.delta);

        window.gl_swap_window();

        app_state.delta = timer.delta();
    }

    Ok(())
}

pub struct AppState<'a> {
    pub input_owner: InputOwner,
    pub winapi: &'a mut QuiPiWindow,
    pub text_render: &'a mut TextRenderer,
    pub delta: f32
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum FrameResponse {
    Quit,
    RelinquishInput,
    #[default] Ignore
}

#[derive(Debug, PartialEq, Eq)]
pub enum InputOwner {
    App,
    Editor
}

fn print_debug(app_state: &AppState, delta: f32) {
    if cfg!(debug_assertions) {
        // text.color = glm::vec3(1.0, 1.0, 1.0);
        // text.scale = 0.7;
        app_state.text_render.draw(
            format!("ms: {}", delta * 1000.0),
            glm::vec2(25.0, 50.0)
        );
        app_state.text_render.draw(
            format!("fps: {}", 1.0 / delta),
            glm::vec2(25.0, 25.0)
        );
    }
}

