use std::collections::HashMap;
use std::collections::HashSet;
use std::mem;

use crate::platform::opengl;
use crate::platform::sdl2;
use crate::plugins::Plugin;
use crate::plugins::Plugins;
use crate::prelude::qp_gfx;
use crate::prelude::qp_gfx::Viewport;
use crate::prelude::QPError;
use crate::prelude::World;
use crate::registry::GlobalRegistry;
use crate::resource_manager::Resource;
use crate::QPResult;

#[cfg(feature = "qp_profiling")]
use crate::prelude::QPProfiler;

pub struct App {
    pub world: World,
    // pub winapi: sdl2::QPWindow,
    #[cfg(feature = "qp_profiling")]
    profiler: QPProfiler,

    runner: Box<dyn FnOnce(App)>,

    plugins: Vec<Box<dyn Plugin>>,
    plugin_names: HashSet<Box<str>>,
    plugins_building_count: usize,

    pub(crate) config: AppConfig,
    pub(crate) state: AppState, // controllers: Vec<Box<dyn Controller>>,
                                // renderers: Vec<Box<dyn Renderer>>,
}

impl App {
    pub fn empty() -> Self {
        Self {
            config: AppConfig::default(),
            world: World::new(),
            runner: Box::new(run_once),
            plugins: vec![],
            plugin_names: HashSet::default(),
            plugins_building_count: 0,

            #[cfg(feature = "qp_profiling")]
            profiler: QPProfiler::new(),
            state: AppState::Created,
        }
    }

    pub fn new() -> Self {
        Self::empty()
    }

    pub fn set_config(&mut self, config: AppConfig) -> &mut Self {
        if self.state != AppState::Created {
            panic!("must load config before anything else")
        }

        self.state = AppState::LoadingConfig;
        self.config = config;

        self
    }

    // pub fn init(title: &str, width: u32, height: u32, seed: u64) -> QPResult<Self> {
    //     // let mut winapi = sdl2::QPWindow::init()?;
    //     // let _window = winapi.opengl_window(title, width, height, (4, 5))?;

    //     // qp_gfx::init(&winapi).map_err(|e| QPError::Generic(e.to_string()))?;

    //     // let viewport = Viewport::new(0, 0, width as i32, height as i32);

    //     // TODO
    //     // let audio = QPAudio::new()?;
    //     // audio.play();

    //     let world = World::new();

    //     Ok(Self {
    //         // winapi,
    //         world,

    //         #[cfg(feature = "qp_profiling")]
    //         profiler: QPProfiler::new(),

    //         runner: Box::new(run_once), // runner: Box::new(|mut app: App| {
    //                                     //     let clear_color = (0.1, 0.1, 0.1, 1.0);

    //                                     //     // 'running: loop {
    //                                     //     let result = app.update(clear_color);
    //                                     //     match result {
    //                                     //         // Ok(FrameResult::Quit) => break 'running,
    //                                     //         Err(e) => {
    //                                     //             eprintln!("App ended unexpectedly: {}", e);

    //                                     //             // break 'running;
    //                                     //         }
    //                                     //         _ => (),
    //                                     //         // }
    //                                     //     }
    //                                     // }),
    //                                     // controllers: vec![],
    //                                     // renderers: vec![],
    //     })
    // }

    pub fn add_plugins(&mut self, plugins: impl Plugins) -> &mut Self {
        self.state = AppState::LoadingPlugins;

        plugins.install(self);

        self
    }

    pub(crate) fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        if let Some(_) = self.plugin_names.get(plugin.name()) {
            panic!("trying to add a duplicate plugin");
        }

        self.plugins_building_count += 1;
        if let Err(e) = plugin.build(self) {
            panic!("There was a problem building the plugins: {}", e);
        };
        self.plugins_building_count -= 1;

        self.plugins.push(plugin);
    }

    pub fn add_startup_system(
        &mut self,
        system: impl FnMut(&mut GlobalRegistry) + 'static,
    ) -> &mut Self {
        self.world.add_startup_system(system);

        self
    }

    pub fn add_system(&mut self, system: impl FnMut(&mut GlobalRegistry) + 'static) -> &mut Self {
        self.world.add_system(system);

        self
    }

    pub fn add_resource(&mut self, resource: impl Resource + 'static) -> &mut Self {
        if let Err(e) = self.world.registry.resources.add_resource(resource) {
            panic!("there was a problem adding a resource: {}", e);
        }

        self
    }

    pub fn add_asset(&mut self) -> &mut Self {
        self
    }

    pub fn set_runner(&mut self, runner: impl FnOnce(App) + 'static) {
        self.runner = Box::new(runner);
    }

    // pub fn register_controller(&mut self, controller: impl Controller + 'static) {
    //     self.controllers.push(Box::new(controller));
    // }

    // pub fn register_renderer(&mut self, renderer: impl Renderer + 'static) {
    //     self.renderers.push(Box::new(renderer));
    // }

    pub fn run(&mut self) -> QPResult<()> {
        self.plugins_done()?;
        self.plugins_cleanup()?;
        self.world.startup();

        let mut app = std::mem::replace(self, App::empty());

        if app.plugins_building_count > 0 {
            panic!("App::run() was called before all plugins were built");
        }

        self.state = AppState::Running;

        let runner = std::mem::replace(&mut app.runner, Box::new(run_once));
        runner(app);

        Ok(())
    }

    fn plugins_done(&mut self) -> QPResult<()> {
        let plugins = std::mem::take(&mut self.plugins);

        for plugin in &plugins {
            plugin.done(self)?;
        }

        self.plugins = plugins;

        Ok(())
    }

    fn plugins_cleanup(&mut self) -> QPResult<()> {
        let plugins = std::mem::take(&mut self.plugins);

        for plugin in &plugins {
            plugin.cleanup(self)?;
        }

        self.plugins = plugins;

        Ok(())
    }

    // fn update(&mut self, clear_color: (f32, f32, f32, f32)) -> QPResult<FrameResult> {
    //     self.world.flush();
    //     self.world.new_frame(&mut self.winapi)?;

    //     opengl::buffer::clear_buffers(clear_color);

    //     // update controllers
    //     #[cfg(feature = "qp_profiling")]
    //     self.profiler.begin();

    //     for controller in self.controllers.iter_mut() {
    //         if controller.update(&mut self.world) == FrameResult::Quit {
    //             return Ok(FrameResult::Quit);
    //         }
    //     }

    //     #[cfg(feature = "qp_profiling")]
    //     {
    //         self.world.debug_info.controller_ms = self.profiler.end() as u32;
    //     }

    //     // call renderers
    //     let mut draw_calls = 0;

    //     #[cfg(feature = "qp_profiling")]
    //     self.profiler.begin();

    //     for renderer in self.renderers.iter_mut() {
    //         if let Some(m_draw_calls) = renderer.draw(&mut self.world) {
    //             draw_calls += m_draw_calls;
    //         }
    //     }

    //     if let Some(window) = &self.winapi.window {
    //         window.gl_swap_window();
    //     } else {
    //         return Err(QPError::ProblemSwappingFrameBuffers);
    //     }

    //     #[cfg(feature = "qp_profiling")]
    //     {
    //         self.world.debug_info.render_ms = self.profiler.end() as u32;
    //     }

    //     self.world.debug_info.draw_calls = draw_calls;

    //     Ok(FrameResult::None)
    // }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FrameResult {
    Quit,
    None,
}

pub trait Controller {
    fn update(&mut self, world: &mut World) -> FrameResult;
}

#[derive(Debug, PartialEq)]
pub enum AppState {
    Created,
    LoadingConfig,
    LoadingPlugins,
    Running,
    Quiting,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            title: "Quipi App".into(),
            width: 800,
            height: 600,
        }
    }
}

fn run_once(mut app: App) {
    app.world.update();
}
