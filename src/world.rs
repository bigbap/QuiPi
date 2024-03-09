use crate::{core::prelude::Timer, platform::sdl2::QPWindow, registry::GlobalRegistry, QPResult};

pub struct World {
    pub registry: GlobalRegistry,

    startup_systems: Vec<Box<dyn FnMut(&mut GlobalRegistry)>>,
    systems: Vec<Box<dyn FnMut(&mut GlobalRegistry)>>,

    pub(crate) state: WorldState, // pub debug_info: DebugInfo,
                                  // pub debug_mode: bool,
                                  // pub events: Vec<Event>,
                                  // pub text_buffer: Vec<QPText>,

                                  // pub viewport: Viewport,
                                  // pub delta: f32,
                                  // timer: Timer,
                                  // pub rand: Random,
}

impl World {
    pub fn new() -> Self {
        let registry = GlobalRegistry::init();
        let mut timer = Timer::new();
        let delta = timer.delta();

        Self {
            registry,
            startup_systems: vec![],
            systems: vec![],
            state: WorldState::Building, // timer,
                                         // delta,
                                         // rand: Random::from_seed(seed),

                                         // debug_info: DebugInfo::default(),
                                         // debug_mode: false,
                                         // events: vec![],
                                         // text_buffer: vec![],

                                         // viewport,
        }
    }

    pub fn add_startup_system(
        &mut self,
        system: impl FnMut(&mut GlobalRegistry) + 'static,
    ) -> &mut Self {
        if self.state != WorldState::Building {
            panic!("system can only be added while in Build state")
        }

        self.startup_systems.push(Box::new(system));

        self
    }

    pub fn add_system(&mut self, system: impl FnMut(&mut GlobalRegistry) + 'static) -> &mut Self {
        if self.state != WorldState::Building {
            panic!("system can only be added while in Build state")
        }

        self.systems.push(Box::new(system));

        self
    }

    pub fn startup(&mut self) {
        for system in self.startup_systems.iter_mut() {
            system(&mut self.registry)
        }

        self.state = WorldState::Ready;
    }

    pub fn update(&mut self) -> bool {
        self.registry.flush();

        for system in self.systems.iter_mut() {
            system(&mut self.registry)
        }

        self.registry.quit
    }

    // pub fn new_frame(&mut self, winapi: &mut QPWindow) -> QPResult<()> {
    //     // self.events = winapi.get_event_queue()?;
    //     // self.delta = self.timer.delta();

    //     // self.debug_info.fps = (1.0 / self.delta) as u32;
    //     // self.debug_info.frame_ms = (self.delta * 1000.0) as u32;

    //     Ok(())
    // }

    // pub fn flush(&mut self) {
    //     self.registry.flush();

    //     // self.text_buffer.clear();
    // }
}

#[derive(Debug, PartialEq)]
pub(crate) enum WorldState {
    Building,
    Ready,
}

#[derive(Debug, Default)]
pub struct DebugInfo {
    pub fps: u32,
    pub frame_ms: u32,

    pub editor_ms: u32,
    pub controller_ms: u32,
    pub render_ms: u32,
    pub draw_calls: u32,
}
