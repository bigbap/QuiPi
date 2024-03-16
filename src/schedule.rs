pub use macros::Schedule;
use std::{any::TypeId, collections::HashMap};

use crate::{
    core::prelude::AnyMap,
    prelude::{Commands, StorageId, SystemId, SystemParams, World},
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

    pub fn add_system<S: Schedule + 'static>(&mut self, system: impl SystemParams) -> &mut Self {
        if let Some(schedule) = self.schedules.get_mut::<S>(&ScheduleId::new::<S>()) {
            schedule.add_system(system);
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
    fn add_system<O, A, S: SystemParams<Out = O, Args = A>>(&mut self, system: S);

    fn update(&mut self, world: &mut World) -> QPResult<()>;
}

pub struct StartupSchedule {
    systems: SystemMap,
}
impl Default for StartupSchedule {
    fn default() -> Self {
        Self {
            systems: SystemMap::new(),
        }
    }
}
impl Schedule for StartupSchedule {
    fn add_system<O, A, S: SystemParams<Out = O, Args = A>>(&mut self, system: S) {
        let id = SystemId::new::<O, A, S>();

        if let Some(systems) = self.systems.get_mut::<Vec<Box<S>>>(&id) {
            systems.push(Box::new(system))
        }
    }

    fn update(&mut self, world: &mut World) -> QPResult<()> {
        Ok(())
    }
}

pub struct UpdateSchedule {
    systems: SystemMap,
}
impl Default for UpdateSchedule {
    fn default() -> Self {
        Self {
            systems: SystemMap::new(),
        }
    }
}
impl Schedule for UpdateSchedule {
    fn add_system<O, A, S: SystemParams<Out = O, Args = A>>(&mut self, system: S) {
        let id = SystemId::new::<O, A, S>();

        if let Some(systems) = self.systems.get_mut::<Vec<Box<S>>>(&id) {
            systems.push(Box::new(system))
        }
    }

    fn update(&mut self, world: &mut World) -> QPResult<()> {
        Ok(())
    }
}

pub struct RenderSchedule {
    systems: SystemMap,
}
impl Default for RenderSchedule {
    fn default() -> Self {
        Self {
            systems: SystemMap::new(),
        }
    }
}
impl Schedule for RenderSchedule {
    fn add_system<O, A, S: SystemParams<Out = O, Args = A>>(&mut self, system: S) {
        let id = SystemId::new::<O, A, S>();

        if let Some(systems) = self.systems.get_mut::<Vec<Box<S>>>(&id) {
            systems.push(Box::new(system))
        }
    }

    fn update(&mut self, world: &mut World) -> QPResult<()> {
        Ok(())
    }
}

// #[derive(Default, Schedule)]
// pub struct StartupSchedule {
//     systems: HashMap<SystemId, Box<dyn std::any::Any>>,
// }

// #[derive(Default, Schedule)]
// pub struct UpdateSchedule {
//     systems: Vec<SystemExecuter>,
// }

// #[derive(Default, Schedule)]
// pub struct RenderSchedule {
//     systems: Vec<SystemExecuter>,
// }

type ScheduleMap = AnyMap<ScheduleId>;
type SystemMap = AnyMap<SystemId>;

#[derive(Eq, PartialEq, Clone, Hash)]
struct ScheduleId(pub TypeId);
impl ScheduleId {
    pub fn new<S: Schedule + 'static>() -> Self {
        Self(std::any::TypeId::of::<S>())
    }
}
