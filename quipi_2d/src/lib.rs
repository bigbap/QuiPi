pub extern crate quipi_core;
pub extern crate nalgebra_glm as glm;
pub extern crate serde;

pub mod components;
pub mod schemas;
pub mod systems;
use quipi_core::{engine::EditorInfo, rendering::RenderInfo};
pub use quipi_core::{
    resources,
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
    systems::text,
    VersionedIndex,
    wrappers::egui::GUI,
    QuiPiApp,
    engine::set_debug_info
};

use components::{
    register_components,
    register_resources,
    CRGBA,
};
use systems::editor::AppEditor;

pub struct QuiPi2D<G: QuiPiApp> {
    app: G,
    registry: Registry,
    winapi: QuiPiWindow,
    timer: Timer,
    frame_state: FrameState,
    app_editor: AppEditor
}

impl<G: QuiPiApp> QuiPi2D<G> {
    pub fn init(
        mut app: G,
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut registry = setup()?;

        let mut winapi = QuiPiWindow::init()?;
        let _window = winapi.opengl_window(
            title,
            width,
            height,
            (4, 5)
        )?;
        
        app.init(&mut registry, &winapi)?;

        let mut timer = Timer::new();
        let frame_state = FrameState {
            clear_color: CRGBA::default(),
            editor_mode: false,
            events: vec![],
            text_render: text::TextRenderer::new(text::DEFAULT_FONT)?,
            render_info: RenderInfo::default(),
            editor_info: EditorInfo::default(),
            debug_info: DebugInfo::default(),
            delta: timer.delta(),
        };

        Ok(Self {
            app,
            registry,
            winapi,
            timer,
            frame_state,
            app_editor: AppEditor::new()?
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // let mut renderer = Renderer2D::new(&mut self.registry);
        'running: loop {
            self.registry.entities.flush();
            self.registry.resources.flush();
    
            clear_buffers(self.frame_state.clear_color.to_tuple());
    
            // // 1. draw all drawables
            // renderer.start()?;
            // renderer.batch_render(CTag { tag: "bubble".into() }, &mut self.registry)?;
            // self.frame_state.render_info = renderer.flush(&self.registry);
    
            // 2. call update systems (any app drawing might happen here. ie rendering text)
            set_debug_info(&mut self.frame_state);
            self.frame_state.events = self.winapi.get_event_queue()?;
            match self.app.handle_frame(&mut self.registry, &mut self.frame_state)? {
                FrameResponse::Quit => break 'running,
                FrameResponse::Restart => { self.timer.delta(); },
                FrameResponse::None => ()
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
    quipi_core::engine::create_asset_dirs()?;

    let mut registry = Registry::init()?;

    register_components(&mut registry);
    register_resources(&mut registry);

    Ok(registry)
}
