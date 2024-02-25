use crate::platform::sdl2;
use crate::platform::opengl;
use crate::QPResult;
use crate::prelude::{
    QPError,
    qp_core::Timer,
    GlobalRegistry,
    qp_data::{
        FrameState,
        FrameResponse,
        IController,
        IRenderer,
        DebugInfo
    },
    qp_gfx::{
        TextRenderer,
        DEFAULT_FONT
    },
    qp_ecs::components::register_components
};

#[cfg(feature = "qp_profiling")]
use crate::prelude::QPProfiler;

pub struct QuiPi {
    pub registry: GlobalRegistry,
    winapi: sdl2::QuiPiWindow,

    #[cfg(feature = "qp_profiling")]
    profiler: QPProfiler,

    frame_timer: Timer,
    frame_state: FrameState,

    controllers: Vec<Box<dyn IController>>,
    renderers: Vec<Box<dyn IRenderer>>
}

impl QuiPi {
    pub fn init(
        title: &str,
        width: u32,
        height: u32,
    ) -> QPResult<Self> {
        let registry = setup()?;

        let mut winapi = sdl2::QuiPiWindow::init()?;
        let _window = winapi.opengl_window(
            title,
            width,
            height,
            (4, 5)
        )?;

        let mut frame_timer = Timer::new();
        let frame_state = FrameState {
            delta: frame_timer.delta(),
            events: vec![],
            text_render: TextRenderer::new(DEFAULT_FONT)?,
            debug_mode: false,
            debug_info: DebugInfo::default(),
        };

        Ok(Self {
            registry,
            winapi,

            #[cfg(feature = "qp_profiling")]
            profiler: QPProfiler::new(),

            frame_timer,
            frame_state,
            controllers: vec![],
            renderers: vec![]
        })
    }

    pub fn register_controller(&mut self, controller: impl IController + 'static) {
        self.controllers.push(Box::new(controller));
    }

    pub fn register_renderer(&mut self, renderer: impl IRenderer + 'static) {
        self.renderers.push(Box::new(renderer));
    }

    pub fn run(&mut self, clear_color: (f32, f32, f32, f32)) -> QPResult<()> {
        'running: loop {
            self.registry.entity_manager.flush();
            self.registry.asset_manager.flush();
    
            set_frame_debug_info(&mut self.frame_state);
            self.frame_state.events = self.winapi.get_event_queue()?;

            // update controllers
            #[cfg(feature = "qp_profiling")]
            self.profiler.begin();

            for controller in self.controllers.iter_mut() {
                match controller.update(&mut self.frame_state, &mut self.registry) {
                    FrameResponse::Quit => break 'running,
                    FrameResponse::Restart => { self.frame_timer.delta(); },
                    FrameResponse::None => ()
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

            opengl::buffer::clear_buffers(clear_color);
            for renderer in self.renderers.iter_mut() {
                if let Some(m_draw_calls) = renderer.draw(
                    &mut self.frame_state,
                    &mut self.registry
                ) {
                    draw_calls += m_draw_calls;
                }
            }

            if let Some(window) = &self.winapi.window {
                window.gl_swap_window();
            } else {
                return Err(QPError::ProblemSwappingFrameBuffers)
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

fn setup() -> QPResult<GlobalRegistry> {
    let mut registry = GlobalRegistry::init()?;

    register_components(&mut registry);

    Ok(registry)
}

pub fn set_frame_debug_info(app_state: &mut FrameState) {
    app_state.debug_info.fps = (1.0 / app_state.delta) as u32;
    app_state.debug_info.frame_ms = (app_state.delta * 1000.0) as u32;
}
