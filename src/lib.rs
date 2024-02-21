pub extern crate sdl2;
pub extern crate gl;
pub extern crate nalgebra_glm as glm;
pub extern crate freetype as ft;
pub extern crate serde;
// pub extern crate gltf;

pub mod registry;
pub mod schemas;
pub mod shaders;
pub mod game_state;
pub mod controllers;

mod core;
mod platform;
mod modules;

pub use modules::ecs::Component;
pub use modules::ecs::VersionedIndex;
pub use modules::*;
pub use registry::Registry;
pub use game_state::*;
pub use core::canvas;
pub use core::math;
pub use core::time;
pub use core::rendering;

use core::rendering::RenderInfo;
use platform::sdl2::window::QuiPiWindow;
use platform::opengl;



pub trait IController {
    fn update(&mut self, _frame_state: &mut FrameState, _registry: &mut Registry) -> FrameResponse { FrameResponse::None }
    fn draw(&mut self, _frame_state: &mut FrameState,  _registry: &mut Registry) -> Option<RenderInfo> { None }
}

pub struct QuiPi {
    pub registry: Registry,
    winapi: QuiPiWindow,
    timer: time::Timer,
    frame_state: FrameState,
    controllers: Vec<Box<dyn IController>>
}

impl QuiPi {
    pub fn init(
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let registry = setup()?;

        #[cfg(feature = "profile-with-tracy")]
        println!("tracy");

        let mut winapi = QuiPiWindow::init()?;
        let _window = winapi.opengl_window(
            title,
            width,
            height,
            (4, 5)
        )?;

        let mut timer = time::Timer::new();
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
            controllers: vec![]
        })
    }

    pub fn register_controller(&mut self, controller: impl IController + 'static) {
        self.controllers.push(Box::new(controller));
    }

    pub fn run(&mut self, clear_color: (f32, f32, f32, f32)) -> Result<(), Box<dyn std::error::Error>> {
        'running: loop {
            self.registry.entities.flush();
            self.registry.flush_resources();
    
            opengl::buffer::clear_buffers(clear_color);
    
            // call controller update and draw
            set_debug_info(&mut self.frame_state);
            self.frame_state.events = self.winapi.get_event_queue()?;
            self.frame_state.render_info = RenderInfo::default();

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
                    self.frame_state.render_info.total_ms += render_info.total_ms;
                    self.frame_state.render_info.num_draw_calls += render_info.num_draw_calls;
                }
            }
            
            // // draw the editor
            // if self.frame_state.editor_mode && cfg!(debug_assertions) {
            //     self.frame_state.editor_info = self.app_editor.update(
            //         &mut self.registry,
            //         &mut self.frame_state
            //     )?;
            // }
            
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

    ecs::components::register_components(&mut registry);
    ecs::resources::register_resources(&mut registry);

    Ok(registry)
}
