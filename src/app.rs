use crate::audio::QPAudio;
use crate::platform::opengl;
use crate::platform::sdl2;
use crate::prelude::qp_gfx::Gfx;
use crate::prelude::{
    qp_core::Timer,
    qp_data::{DebugInfo, FrameState, IController, IRenderer, Viewport},
    qp_ecs::components::register_components,
    qp_gfx::TextRenderer,
    GlobalRegistry, QPError,
};
use crate::QPResult;

#[cfg(feature = "qp_profiling")]
use crate::prelude::QPProfiler;

pub struct App {
    pub registry: GlobalRegistry,
    winapi: sdl2::QPWindow,

    #[cfg(feature = "qp_profiling")]
    profiler: QPProfiler,

    frame_timer: Timer,
    frame_state: FrameState,

    controllers: Vec<Box<dyn IController>>,
    renderers: Vec<Box<dyn IRenderer>>,
}

impl App {
    pub fn init(title: &str, width: u32, height: u32) -> QPResult<Self> {
        let mut winapi = sdl2::QPWindow::init()?;
        let _window = winapi.opengl_window(title, width, height, (4, 5))?;
        let gfx = Gfx::init(&winapi, width as i32, height as i32)
            .map_err(|e| QPError::Generic(e.to_string()))?;

        let mut registry = GlobalRegistry::init(gfx)?;
        register_components(&mut registry);

        let audio = QPAudio::new()?;
        audio.play();

        let mut frame_timer = Timer::new();
        let frame_state = FrameState {
            delta: frame_timer.delta(),
            debug_mode: false,
            debug_info: DebugInfo::default(),
            viewport: Viewport {
                width: width as i32,
                height: height as i32,
            },
        };

        Ok(Self {
            registry,
            winapi,

            #[cfg(feature = "qp_profiling")]
            profiler: QPProfiler::new(),

            frame_timer,
            frame_state,
            controllers: vec![],
            renderers: vec![],
        })
    }

    pub fn register_controller(&mut self, controller: impl IController + 'static) {
        self.controllers.push(Box::new(controller));
    }

    pub fn register_renderer(&mut self, renderer: impl IRenderer + 'static) {
        self.renderers.push(Box::new(renderer));
    }

    pub fn run(&mut self, clear_color: (f32, f32, f32, f32)) -> QPResult<()> {
        self.register_renderer(TextRenderer::new()?);

        'running: loop {
            self.registry.flush();
            self.registry.new_frame(&self.winapi)?;

            set_frame_debug_info(&mut self.frame_state);

            opengl::buffer::clear_buffers(clear_color);

            // update controllers
            #[cfg(feature = "qp_profiling")]
            self.profiler.begin();

            for controller in self.controllers.iter_mut() {
                match controller.update(&mut self.frame_state, &mut self.registry) {
                    FrameResult::Quit => break 'running,
                    FrameResult::Restart => {
                        self.frame_timer.delta();
                    }
                    FrameResult::None => (),
                }
            }

            #[cfg(feature = "qp_profiling")]
            {
                self.frame_state.debug_info.controller_ms = self.profiler.end() as u32;
            }

            // call renderers
            let mut draw_calls = 0;

            #[cfg(feature = "qp_profiling")]
            self.profiler.begin();

            for renderer in self.renderers.iter_mut() {
                if let Some(m_draw_calls) = renderer.draw(&mut self.frame_state, &mut self.registry)
                {
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
                self.frame_state.debug_info.render_ms = self.profiler.end() as u32;
            }

            self.frame_state.debug_info.draw_calls = draw_calls;
            self.frame_state.delta = self.frame_timer.delta();
        }

        Ok(())
    }
}

pub fn set_frame_debug_info(app_state: &mut FrameState) {
    app_state.debug_info.fps = (1.0 / app_state.delta) as u32;
    app_state.debug_info.frame_ms = (app_state.delta * 1000.0) as u32;
}

#[derive(Debug)]
pub struct World {
    pub registry: GlobalRegistry,
    pub delta: f32,
    pub debug_mode: bool,
    pub debug_info: DebugInfo,
    pub viewport: Viewport,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FrameResult {
    Quit,
    None,
    Restart,
}
