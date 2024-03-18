pub use macros::Schedule;
use std::collections::HashMap;

use crate::{
    prelude::{BoxedSystem, IntoSystem, World},
    resources::{AsAny, Resource},
};

#[derive(Resource, AsAny)]
pub struct ScheduleManager {
    schedules: HashMap<u64, Schedule>,
}

impl ScheduleManager {
    pub fn new() -> Self {
        Self {
            schedules: HashMap::new(),
        }
    }

    pub fn insert_schedule(&mut self, schedule: impl ScheduleLabel) -> &mut Self {
        self.schedules.insert(schedule.id(), Schedule::new());

        self
    }

    pub fn add_system<M, S: IntoSystem<(), M>>(
        &mut self,
        schedule: impl ScheduleLabel,
        system: S,
    ) -> &mut Self {
        if let Some(schedule) = self.schedules.get_mut(&schedule.id()) {
            schedule.add_system::<M, S>(system);
        } else {
            #[cfg(debug_assertions)]
            println!("trying to add system to a schedule that doesn't exist");
        }

        self
    }

    pub(crate) fn execute_schedule(&mut self, schedule: impl ScheduleLabel, world: &mut World) {
        if let Some(schedule) = self.schedules.get_mut(&schedule.id()) {
            schedule.update(world);
        } else {
            #[cfg(debug_assertions)]
            println!("trying to update a schedule that doesn't exist");
        }
    }
}

pub struct Schedule {
    systems: Vec<BoxedSystem>,
}

impl Schedule {
    pub fn new() -> Self {
        Self { systems: vec![] }
    }

    fn add_system<M, S: IntoSystem<(), M>>(&mut self, system: S) {
        self.systems.push(Box::new(IntoSystem::into_system(system)))
    }

    fn update(&mut self, world: &mut World) {
        for system in self.systems.iter_mut() {
            system.run(world);
        }
    }
}

pub trait ScheduleLabel {
    const ID: u64;

    fn id(&self) -> u64;
}

pub struct Startup;
impl ScheduleLabel for Startup {
    const ID: u64 = 0x457DE0A607086830;

    fn id(&self) -> u64 {
        Self::ID
    }
}

pub struct Update;
impl ScheduleLabel for Update {
    const ID: u64 = 0x7F81D58E83F97AC8;

    fn id(&self) -> u64 {
        Self::ID
    }
}

pub struct Render;
impl ScheduleLabel for Render {
    const ID: u64 = 0x5CEA66B00E5350D4;

    fn id(&self) -> u64 {
        Self::ID
    }
}

pub struct Cleanup;
impl ScheduleLabel for Cleanup {
    const ID: u64 = 0x4589090E7A6B3E03;

    fn id(&self) -> u64 {
        Self::ID
    }
}
