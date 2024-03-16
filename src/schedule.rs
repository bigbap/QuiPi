pub use macros::Schedule;
use std::any::TypeId;

use crate::{
    core::prelude::AnyMap,
    prelude::{BoxedSystem, IntoSystem, SystemParam, World},
    resources::{AsAny, Resource},
    QPResult,
};

#[derive(Resource, AsAny)]
pub struct ScheduleManager {
    // schedules: HashMap<&'static str, Box<dyn Schedule>>,
    schedules: ScheduleMap,
}

impl ScheduleManager {
    pub fn new() -> Self {
        Self {
            schedules: ScheduleMap::new(),
        }
    }

    pub fn add_schedule<S: Schedule + Default + 'static>(&mut self) -> &mut Self {
        self.schedules
            .insert::<S>(ScheduleId::new::<S>(), S::default());

        self
    }

    pub fn add_system<S, System, Params>(&mut self, system: System) -> &mut Self
    where
        S: Schedule + 'static,
        System: IntoSystem<Params>,
        Params: SystemParam + 'static,
    {
        if let Some(schedule) = self.schedules.get_mut::<S>(&ScheduleId::new::<S>()) {
            schedule.add_system::<S, System, Params>(system);
        } else {
            #[cfg(debug_assertions)]
            println!("trying to add system to a schedule that doesn't exist");
        }

        self
    }

    pub(crate) fn execute_schedule<S: Schedule + 'static>(
        &mut self,
        world: &mut World,
    ) -> QPResult<()> {
        if let Some(schedule) = self.schedules.get_mut::<S>(&ScheduleId::new::<S>()) {
            schedule.update(world)?;
        } else {
            #[cfg(debug_assertions)]
            println!("trying to update a schedule that doesn't exist");
        }

        Ok(())
    }
}

pub trait Schedule {
    fn add_system<S, System, Params>(&mut self, system: System)
    where
        S: Schedule + 'static,
        System: IntoSystem<Params>,
        Params: SystemParam + 'static;

    fn update(&mut self, world: &mut World) -> QPResult<()>;
}

pub struct StartupSchedule {
    systems: Vec<BoxedSystem>,
}
impl Default for StartupSchedule {
    fn default() -> Self {
        Self { systems: vec![] }
    }
}
impl Schedule for StartupSchedule {
    fn add_system<S, System, Params>(&mut self, system: System)
    where
        S: Schedule + 'static,
        System: IntoSystem<Params>,
        Params: SystemParam + 'static,
    {
        self.systems.push(Box::new(system.into_system()))
    }

    fn update(&mut self, world: &mut World) -> QPResult<()> {
        for system in self.systems.iter_mut() {
            system.run(world)?;
        }

        Ok(())
    }
}

#[derive(Schedule)]
pub struct UpdateSchedule {
    systems: Vec<BoxedSystem>,
}
impl Default for UpdateSchedule {
    fn default() -> Self {
        Self { systems: vec![] }
    }
}

#[derive(Schedule)]
pub struct RenderSchedule {
    systems: Vec<BoxedSystem>,
}
impl Default for RenderSchedule {
    fn default() -> Self {
        Self { systems: vec![] }
    }
}

type ScheduleMap = AnyMap<ScheduleId>;

#[derive(Eq, PartialEq, Clone, Hash)]
struct ScheduleId(pub TypeId);
impl ScheduleId {
    pub fn new<S: Schedule + 'static>() -> Self {
        Self(std::any::TypeId::of::<S>())
    }
}
