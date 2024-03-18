use std::marker::PhantomData;

use crate::{
    assets::AssetServer,
    core::prelude::StringInterner,
    ecs::prelude::StorageManager,
    prelude::{QPError, ScheduleManager},
    resources::{Resource, ResourceManager},
    schedule::ScheduleLabel,
    QPResult,
};

pub use super::commands::Commands;

pub struct World {
    pub resources: ResourceManager,
    pub(crate) quitting: bool,
}

impl World {
    pub fn new() -> Self {
        Self {
            resources: ResourceManager::new(),
            quitting: false,
        }
    }

    pub(crate) fn execute(&mut self, schedule: impl ScheduleLabel) -> QPResult<()> {
        let mut schedules = self.resources.remove_or_err::<ScheduleManager>()?;

        schedules
            .borrow_mut::<ScheduleManager>()
            .ok_or(QPError::Generic(
                "couldn't borrow ScheduleManager from ResourceOwner".into(),
            ))?
            .execute_schedule(schedule, self);

        self.resources.insert_owner(schedules)?;

        Ok(())
    }

    pub fn resource<R: Resource + 'static>(&self) -> Option<&R> {
        self.resources.get::<R>()
    }

    pub fn resource_mut<R: Resource + 'static>(&mut self) -> Option<&mut R> {
        self.resources.get_mut::<R>()
    }

    pub fn quit(&mut self) {
        self.quitting = true
    }

    // MANDATORY RESOURCES
    //
    // if you try to call these methods before the resources
    // have been loaded, they will panic

    // StorageManager

    pub fn storage_manager(&self) -> &StorageManager {
        match self.resource::<StorageManager>() {
            Some(storage) => storage,
            _ => panic!("Storage manager not found"),
        }
    }

    pub fn storage_manager_mut(&mut self) -> &mut StorageManager {
        match self.resource_mut::<StorageManager>() {
            Some(storage) => storage,
            _ => panic!("Storage manager not found"),
        }
    }

    // AssetServer

    pub fn asset_server(&mut self) -> &mut AssetServer {
        match self.resource_mut::<AssetServer>() {
            Some(assets) => assets,
            _ => panic!("Asset server not found"),
        }
    }

    // StringInterner

    pub fn interner(&self) -> &StringInterner {
        match self.resource::<StringInterner>() {
            Some(interner) => interner,
            _ => panic!("String interner not found"),
        }
    }

    pub fn interner_mut(&mut self) -> &mut StringInterner {
        match self.resource_mut::<StringInterner>() {
            Some(interner) => interner,
            _ => panic!("String interner not found"),
        }
    }

    // Schedules

    pub fn schedule_manager(&self) -> &ScheduleManager {
        match self.resource::<ScheduleManager>() {
            Some(schedule) => schedule,
            _ => panic!("ScheduleManager not found"),
        }
    }

    pub fn schedule_manager_mut(&mut self) -> &mut ScheduleManager {
        match self.resource_mut::<ScheduleManager>() {
            Some(schedule) => schedule,
            _ => panic!("ScheduleManager not found"),
        }
    }

    // UNSAFE
    // get unsafe cells for world

    #[allow(unused)]
    #[inline]
    pub(crate) fn as_unsafe_cell(&self) -> UnsafeWorldCell<'_> {
        UnsafeWorldCell::new(self)
    }

    #[inline]
    pub(crate) fn as_unsafe_cell_mut(&mut self) -> UnsafeWorldCell<'_> {
        UnsafeWorldCell::new_mut(self)
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

#[derive(Clone, Copy)]
pub struct UnsafeWorldCell<'w>(*mut World, PhantomData<&'w World>);

impl<'w> UnsafeWorldCell<'w> {
    #[allow(unused)]
    #[inline]
    pub(crate) fn new(world: &'w World) -> Self {
        Self(std::ptr::from_ref(world).cast_mut(), PhantomData)
    }

    #[inline]
    pub(crate) fn new_mut(world: &'w mut World) -> Self {
        Self(std::ptr::from_mut(world), PhantomData)
    }

    #[inline]
    pub unsafe fn world(self) -> &'w World {
        unsafe { &*self.0 }
    }

    #[inline]
    pub unsafe fn world_mut(self) -> &'w mut World {
        unsafe { &mut *self.0 }
    }
}
