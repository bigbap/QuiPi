use sdl2::event::Event;

use crate::{
    core::prelude::{random::Random, Timer},
    platform::sdl2::QPWindow,
    prelude::qp_gfx::{QPText, Viewport},
    registry::GlobalRegistry,
    QPResult,
};

pub struct World {
    pub registry: GlobalRegistry,
    pub debug_info: DebugInfo,
    pub debug_mode: bool,

    pub events: Vec<Event>,
    pub text_buffer: Vec<QPText>,

    pub viewport: Viewport,

    pub delta: f32,
    timer: Timer,
    pub rand: Random,
}

impl World {
    pub fn new(viewport: Viewport, seed: u64) -> QPResult<Self> {
        let registry = GlobalRegistry::init()?;
        let mut timer = Timer::new();
        let delta = timer.delta();

        Ok(Self {
            registry,
            timer,
            delta,
            rand: Random::from_seed(seed),

            debug_info: DebugInfo::default(),
            debug_mode: false,

            events: vec![],
            text_buffer: vec![],

            viewport,
        })
    }

    pub fn reset(&self) {
        todo!()
    }

    pub fn new_frame(&mut self, winapi: &mut QPWindow) -> QPResult<()> {
        self.events = winapi.get_event_queue()?;
        self.delta = self.timer.delta();

        self.debug_info.fps = (1.0 / self.delta) as u32;
        self.debug_info.frame_ms = (self.delta * 1000.0) as u32;

        Ok(())
    }

    pub fn flush(&mut self) {
        self.registry.flush();

        self.text_buffer.clear();
    }
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
