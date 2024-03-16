use crate::{
    common::resources::StringInterner,
    ecs::prelude::{Bundle, Index, StorageId, StorageManager},
    prelude::{QPError, ScheduleManager},
    resources::{Resource, ResourceManager},
    schedule::Schedule,
    QPResult,
};

pub struct World {
    pub resources: ResourceManager,
    // pub(crate) schedules: ScheduleManager,
}

impl World {
    pub fn new() -> Self {
        Self {
            resources: ResourceManager::new(),
            // schedules: ScheduleManager::new(),
        }
    }

    pub(crate) fn execute<S: Schedule>(&mut self) -> QPResult<()> {
        let mut schedules = self.resources.remove_or_err::<ScheduleManager>()?;

        schedules
            .borrow_mut::<ScheduleManager>()
            .ok_or(QPError::Generic(
                "couldn't borrow ScheduleManager from ResourceOwner".into(),
            ))?
            .execute_schedule::<S>(self)?;

        self.resources.insert_owner(schedules)?;

        Ok(())
    }

    pub fn spawn(&mut self, storage: StorageId, bundle: impl Bundle) -> QPResult<Index> {
        Ok(self
            .resources
            .get_mut::<StorageManager>()
            .ok_or(QPError::ResourceNotFound("Storage Manager".into()))?
            .create(storage, bundle)
            .ok_or(QPError::Generic("Storage unit not found".into()))?)
    }

    pub fn resource<R: Resource + 'static>(&self) -> Option<&R> {
        self.resources.get::<R>()
    }

    pub fn resource_mut<R: Resource + 'static>(&mut self) -> Option<&mut R> {
        self.resources.get_mut::<R>()
    }

    pub fn intern(&mut self, string: &str) -> QPResult<u64> {
        Ok(self
            .resource_mut::<StringInterner>()
            .ok_or(QPError::ResourceNotFound("String Interner".into()))?
            .intern(string.into()))
    }

    pub fn get_string(&self, key: u64) -> Option<String> {
        self.resource::<StringInterner>()?.get_string(key)
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

// #[derive(Clone, Copy)]
// pub struct UnsafeWorldCell(pub *mut World);

// impl UnsafeWorldCell {
//     #[inline]
//     pub(crate) fn new_readonly(world: &'static World) -> Self {
//         Self(std::ptr::from_ref(world).cast_mut())
//     }

//     #[inline]
//     pub(crate) fn new_mutable(world: &'static mut World) -> Self {
//         Self(ptr::from_mut(world))
//     }
// }
