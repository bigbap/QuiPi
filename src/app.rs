use crate::platform::opengl;
use crate::platform::sdl2;
use crate::prelude::qp_gfx;
use crate::prelude::qp_gfx::Viewport;
use crate::prelude::World;
use crate::prelude::{qp_gfx::TextRenderer, QPError};
use crate::QPResult;

#[cfg(feature = "qp_profiling")]
use crate::prelude::QPProfiler;

pub struct App {
    pub world: World,
    pub winapi: sdl2::QPWindow,

    #[cfg(feature = "qp_profiling")]
    profiler: QPProfiler,

    controllers: Vec<Box<dyn Controller>>,
    renderers: Vec<Box<dyn Renderer>>,
}

impl App {
    pub fn init(title: &str, width: u32, height: u32, seed: u64) -> QPResult<Self> {
        let mut winapi = sdl2::QPWindow::init()?;
        let _window = winapi.opengl_window(title, width, height, (4, 5))?;

        qp_gfx::init(&winapi).map_err(|e| QPError::Generic(e.to_string()))?;

        let viewport = Viewport::new(0, 0, width as i32, height as i32);

        // TODO
        // let audio = QPAudio::new()?;
        // audio.play();

        let world = World::new(viewport, seed)?;

        Ok(Self {
            winapi,
            world,

            #[cfg(feature = "qp_profiling")]
            profiler: QPProfiler::new(),

            controllers: vec![],
            renderers: vec![],
        })
    }

    pub fn register_controller(&mut self, controller: impl Controller + 'static) {
        self.controllers.push(Box::new(controller));
    }

    pub fn register_renderer(&mut self, renderer: impl Renderer + 'static) {
        self.renderers.push(Box::new(renderer));
    }

    pub fn run(&mut self, clear_color: (f32, f32, f32, f32)) -> QPResult<()> {
        self.register_renderer(TextRenderer::new()?);

        'running: loop {
            self.world.flush();
            self.world.new_frame(&mut self.winapi)?;

            opengl::buffer::clear_buffers(clear_color);

            // update controllers
            #[cfg(feature = "qp_profiling")]
            self.profiler.begin();

            for controller in self.controllers.iter_mut() {
                match controller.update(&mut self.world) {
                    FrameResult::Quit => break 'running,
                    FrameResult::Restart => {
                        self.world.reset();
                    }
                    FrameResult::None => (),
                }
            }

            #[cfg(feature = "qp_profiling")]
            {
                self.world.debug_info.controller_ms = self.profiler.end() as u32;
            }

            // call renderers
            let mut draw_calls = 0;

            #[cfg(feature = "qp_profiling")]
            self.profiler.begin();

            for renderer in self.renderers.iter_mut() {
                if let Some(m_draw_calls) = renderer.draw(&mut self.world) {
                    draw_calls += m_draw_calls;
                }
            }

            if let Some(window) = &self.winapi.window {
                window.gl_swap_window();
            } else {
                return Err(QPError::ProblemSwappingFrameBuffers);
            }

            #[cfg(feature = "qp_profiling")]
            {
                self.world.debug_info.render_ms = self.profiler.end() as u32;
            }

            self.world.debug_info.draw_calls = draw_calls;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FrameResult {
    Quit,
    None,
    Restart,
}

pub trait Renderer {
    fn draw(&mut self, world: &mut World) -> Option<u32>;
}

pub trait Controller {
    fn update(&mut self, world: &mut World) -> FrameResult;
}
