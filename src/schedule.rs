pub use macros::Schedule;
use std::collections::HashMap;

use crate::{
    prelude::World,
    resources::{AsAny, Resource},
    QPResult,
};

#[derive(Resource, AsAny)]
pub struct ScheduleManager {
    schedules: HashMap<&'static str, Box<dyn Schedule>>,
}

impl ScheduleManager {
    pub fn new() -> Self {
        Self {
            schedules: HashMap::new(),
        }
    }

    pub fn add_schedule<S: Schedule + Default + 'static>(&mut self) -> &mut Self {
        self.schedules
            .insert(std::any::type_name::<S>(), Box::new(S::default()));

        self
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

    pub(crate) fn execute_schedule<S: Schedule>(&mut self, world: &mut World) -> QPResult<()> {
        let name = std::any::type_name::<S>();

        if let Some(schedule) = self.schedules.get_mut(&name) {
            schedule.update(world)?;
        } else {
            #[cfg(debug_assertions)]
            println!("trying to update a schedule that doesn't exist: {:?}", name);
        }

        Ok(())
    }
}

pub trait System: FnMut(&mut World) -> QPResult<()> + 'static {}
impl<F> System for F where F: FnMut(&mut World) -> QPResult<()> + 'static {}

pub type BoxedSystem = Box<dyn System>;

pub trait Schedule {
    fn add_system(&mut self, system: BoxedSystem);

    fn update(&mut self, world: &mut World) -> QPResult<()>;
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
