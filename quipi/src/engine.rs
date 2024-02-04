use std::fs;

use sdl2::event::Event;

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
        egui::GUI,
        opengl::buffer::clear_buffers
    },
    core::time::Timer, utils::to_abs_path
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
    let mut gui = GUI::new(1.0)?;
    let mut timer = Timer::new()?;

    gui.add_ui_region("My GUI Window", false, |ui| {
        ui.add(egui::Label::new("Hello World!"));
        ui.label("This is a label");
        if ui.button("Click me").clicked() {
            println!("egui was clicked");
        }
    });

    gui.add_ui_region("My GUI Window 2", false, |ui| {
        let mut some_bool = true;
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0; // remove spacing between widgets
            // `radio_value` also works for enums, integers, and more.
            ui.radio_value(&mut some_bool, false, "Off");
            ui.radio_value(&mut some_bool, true, "On");
        });
    });

    let mut app_state = AppState {
        editor_mode: false,
        events: vec![],
        text_render: &mut text,
        delta: timer.delta(),
    };

    'running: loop {
        clear_buffers((0.4, 0.4, 0.4, 1.0));
        app_state.events = winapi.get_event_queue()?;

        match game.handle_frame(&mut app_state)? {
            FrameResponse::Quit => break 'running,
            FrameResponse::Restart => { timer.delta(); },
            FrameResponse::None => ()
        }

        match gui.update(&mut app_state)? {
            FrameResponse::Quit => break 'running,
            FrameResponse::Restart => { timer.delta(); },
            FrameResponse::None => ()
        }

        print_debug(&app_state, app_state.delta);

        window.gl_swap_window();

        app_state.delta = timer.delta();
    }

    Ok(())
}

pub struct AppState<'a> {
    pub editor_mode: bool,
    pub events: Vec<Event>,
    pub text_render: &'a mut TextRenderer,
    pub delta: f32,
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

fn boilerplate() -> Result<(), Box<dyn std::error::Error>> {
    let asset_path = to_abs_path("assets")?;
    
    fs::create_dir_all(format!("{}/scenes", asset_path))?;
    fs::create_dir_all(format!("{}/shaders", asset_path))?;
    fs::create_dir_all(format!("{}/objects", asset_path))?;
    fs::create_dir_all(format!("{}/fonts", asset_path))?;

    Ok(())
}
