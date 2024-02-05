use std::fs;

use sdl2::event::Event;

use crate::{
    systems::{
        self,
        rendering::text::{
            DEFAULT_FONT,
            TextRenderer
        }, editor::SceneEditor
    },
    wrappers::{
        sdl2::window::QuiPiWindow,
        opengl::buffer::clear_buffers
    },
    core::time::Timer, utils::to_abs_path, components::CRGBA
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

    /// TODO
    fn handle_editor(
        &mut self,
        app_state: &AppState,
        editor: &mut SceneEditor
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub fn run<G: QuiPiApp>(
    game: &mut G,
    title: &str,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    boilerplate()?;

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
    let mut scene_editor = SceneEditor::new()?;
    let mut timer = Timer::new()?;


    let mut app_state = AppState {
        clear_color: CRGBA::default(),
        editor_mode: false,
        events: vec![],
        text_render: &mut text,
        debug_info: DebugInfo::default(),
        delta: timer.delta(),
    };

    'running: loop {
        clear_buffers(app_state.clear_color.to_tuple());
        app_state.events = winapi.get_event_queue()?;

        set_debug_info(&mut app_state);

        match game.handle_frame(&mut app_state)? {
            FrameResponse::Quit => break 'running,
            FrameResponse::Restart => { timer.delta(); },
            FrameResponse::None => ()
        }
        
        if app_state.editor_mode {
            game.handle_editor(&app_state, &mut scene_editor)?;
        }

        window.gl_swap_window();

        app_state.delta = timer.delta();
    }

    Ok(())
}

pub struct AppState<'a> {
    pub clear_color: CRGBA,
    pub editor_mode: bool,
    pub events: Vec<Event>,
    pub text_render: &'a mut TextRenderer,
    pub debug_info: DebugInfo,
    pub delta: f32,
}

#[derive(Debug, Default)]
pub struct DebugInfo {
    pub fps: f32,
    pub ms: f32
}

#[derive(Debug, Default)]
pub struct QPMouseState {
    pub pos: glm::Vec2,
    pub rel_pos: glm::Vec2,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FrameResponse {
    Quit,
    None,
    Restart
}

#[derive(Debug, PartialEq, Eq)]
pub enum AppMode {
    App,
    Editor
}

fn set_debug_info(app_state: &mut AppState) {
    app_state.debug_info.fps = 1.0 / app_state.delta;
    app_state.debug_info.ms = app_state.delta * 1000.0;
}

fn boilerplate() -> Result<(), Box<dyn std::error::Error>> {
    let asset_path = to_abs_path("assets")?;
    
    fs::create_dir_all(format!("{}/scenes", asset_path))?;
    fs::create_dir_all(format!("{}/shaders", asset_path))?;
    fs::create_dir_all(format!("{}/objects", asset_path))?;
    fs::create_dir_all(format!("{}/fonts", asset_path))?;

    Ok(())
}
