pub extern crate quipi_core;
pub extern crate nalgebra_glm as glm;
pub extern crate serde;

pub mod components;
pub mod resources;
pub mod schemas;
pub mod systems;
use quipi_core::{EditorInfo, rendering::RenderInfo};
pub use quipi_core::{
    IController,
    DebugInfo,
    FrameResponse,
    FrameState,
    QuiPiWindow,
    Registry,
    utils::Timer,
    core,
    rendering,
    opengl::{
        buffer::clear_buffers,
        draw::DrawMode
    },
    core::text,
    VersionedIndex,
    platform::egui::GUI,
    set_debug_info
};

use components::register_components;
use resources::register_resources;
use systems::editor::AppEditor;

pub struct QuiPi2D {
    pub registry: Registry,
    winapi: QuiPiWindow,
    timer: Timer,
    frame_state: FrameState,
    app_editor: AppEditor,
    controllers: Vec<Box<dyn IController>>
}

impl QuiPi2D {
    pub fn init(
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let registry = setup()?;

        let mut winapi = QuiPiWindow::init()?;
        let _window = winapi.opengl_window(
            title,
            width,
            height,
            (4, 5)
        )?;

        let mut timer = Timer::new();
        let frame_state = FrameState {
            clear_color: glm::vec4(0.3, 0.5, 0.1, 1.0),
            editor_mode: false,
            events: vec![],
            text_render: text::TextRenderer::new(text::DEFAULT_FONT)?,
            render_info: RenderInfo::default(),
            editor_info: EditorInfo::default(),
            debug_info: DebugInfo::default(),
            delta: timer.delta(),
        };

        Ok(Self {
            registry,
            winapi,
            timer,
            frame_state,
            app_editor: AppEditor::new()?,
            controllers: vec![]
        })
    }

    pub fn register_controller(&mut self, controller: impl IController + 'static) {
        self.controllers.push(Box::new(controller));
    }

    pub fn run(&mut self, clear_color: (f32, f32, f32, f32)) -> Result<(), Box<dyn std::error::Error>> {
        // let mut renderer = Renderer2D::new(&mut self.registry);
        'running: loop {
            self.registry.entities.flush();
            self.registry.flush_resources();
    
            clear_buffers(clear_color);
    
            // call controller update and draw
            set_debug_info(&mut self.frame_state);
            self.frame_state.events = self.winapi.get_event_queue()?;

            for controller in self.controllers.iter_mut() {
                match controller.update(&mut self.frame_state, &mut self.registry) {
                    FrameResponse::Quit => break 'running,
                    FrameResponse::Restart => { self.timer.delta(); },
                    FrameResponse::None => ()
                }

                if let Some(render_info) = controller.draw(
                    &mut self.frame_state,
                    &mut self.registry
                ) {
                    self.frame_state.render_info = render_info;
                }
            }
            
            // draw the editor
            if self.frame_state.editor_mode && cfg!(debug_assertions) {
                self.frame_state.editor_info = self.app_editor.update(
                    &mut self.registry,
                    &mut self.frame_state
                )?;
            }
            
            if let Some(window) = &self.winapi.window {
                window.gl_swap_window();
            } else {
                return Err("There was a problem drawing the frame".into())
            }
    
            self.frame_state.delta = self.timer.delta();
        }
    
        Ok(())
    }
}

fn setup() -> Result<Registry, Box<dyn std::error::Error>> {
    let mut registry = Registry::init()?;

    register_components(&mut registry);
    register_resources(&mut registry);

    Ok(registry)
}
