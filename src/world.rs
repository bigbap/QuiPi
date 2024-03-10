use std::collections::HashMap;

pub use macros::Schedule;

use crate::{registry::GlobalRegistry, QPResult};

pub struct World {
    pub registry: GlobalRegistry,

    schedules: HashMap<&'static str, Box<dyn Schedule>>,
}

impl World {
    pub fn new() -> Self {
        let registry = GlobalRegistry::init();

        Self {
            registry,
            schedules: HashMap::<&'static str, Box<dyn Schedule>>::new(),
        }
    }

    pub fn add_schedule<S: Schedule + Default + 'static>(&mut self) {
        self.schedules
            .insert(std::any::type_name::<S>(), Box::new(S::default()));
    }

    pub fn add_system<S: Schedule>(&mut self, system: impl System) -> &mut Self {
        let name = std::any::type_name::<S>();

        if let Some(schedule) = self.schedules.get_mut(&name) {
            schedule.add_system(Box::new(system));
        } else {
            #[cfg(debug_assertions)]
            println!(
                "trying to add system to a schedule that doesn't exist: {:?}",
                name
            );
        }

        self
    }

    pub fn execute_schedule<S: Schedule>(&mut self) -> QPResult<()> {
        let name = std::any::type_name::<S>();

        if let Some(schedule) = self.schedules.get_mut(&name) {
            schedule.update(&mut self.registry)?;
        } else {
            #[cfg(debug_assertions)]
            println!("trying to update a schedule that doesn't exist: {:?}", name);
        }

        Ok(())
    }
}

pub trait System: FnMut(&mut GlobalRegistry) -> QPResult<()> + 'static {}
impl<F> System for F where F: FnMut(&mut GlobalRegistry) -> QPResult<()> + 'static {}

pub type BoxedSystem = Box<dyn System>;

pub trait Schedule {
    fn add_system(&mut self, system: BoxedSystem);

    fn update(&mut self, registry: &mut GlobalRegistry) -> QPResult<()>;
}

#[derive(Default, Schedule)]
pub struct StartupSchedule {
    systems: Vec<BoxedSystem>,
}

#[derive(Default, Schedule)]
pub struct UpdateSchedule {
    systems: Vec<BoxedSystem>,
}

#[derive(Default, Schedule)]
pub struct RenderSchedule {
    systems: Vec<BoxedSystem>,
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
