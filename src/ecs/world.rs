use crate::{
    assets::AssetServer,
    common::resources::StringInterner,
    ecs::prelude::StorageManager,
    prelude::{QPError, ScheduleManager},
    resources::{Resource, ResourceManager},
    schedule::Schedule,
    QPResult,
};

pub use super::commands::Commands;

pub struct World {
    pub resources: ResourceManager,
}

impl World {
    pub fn new() -> Self {
        Self {
            resources: ResourceManager::new(),
        }
    }

    pub(crate) fn execute<S: Schedule + 'static>(&mut self) -> QPResult<()> {
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

    pub fn resource<R: Resource + 'static>(&self) -> Option<&R> {
        self.resources.get::<R>()
    }

    pub fn resource_mut<R: Resource + 'static>(&mut self) -> Option<&mut R> {
        self.resources.get_mut::<R>()
    }

    // MANDATORY RESOURCES
    //
    // if you try to call these methods before the resources
    // have been loaded, they will panic

    pub fn storage(&self) -> &StorageManager {
        match self.resource::<StorageManager>() {
            Some(storage) => storage,
            _ => panic!("Storage manager not found"),
        }
    }

    pub fn storage_mut(&mut self) -> &mut StorageManager {
        match self.resource_mut::<StorageManager>() {
            Some(storage) => storage,
            _ => panic!("Storage manager not found"),
        }
    }

    pub fn assets(&self) -> &AssetServer {
        match self.resource::<AssetServer>() {
            Some(assets) => assets,
            _ => panic!("Asset server not found"),
        }
    }

    pub fn assets_mut(&mut self) -> &mut AssetServer {
        match self.resource_mut::<AssetServer>() {
            Some(assets) => assets,
            _ => panic!("Asset server not found"),
        }
    }

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
//         Self(std::ptr::from_mut(world))
//     }
// }
