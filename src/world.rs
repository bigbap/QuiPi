use crate::{common::resources::{Asset, AssetId, AssetLoader, AssetStore, Camera, CameraId, CameraList, StringInterner}, prelude::QPError, resources::ResourceManager, storage::prelude::{Bundle, Index, StorageId, StorageManager}, QPResult};

pub struct World {
    pub resources: ResourceManager,
}

impl World {
    pub fn new() -> Self {
        Self {
            resources: ResourceManager::new(),
        }
    }

    /// ///////////
    ///
    /// Assets
    ///
    /// ///////////

    pub fn load_asset<A: Asset + 'static>(
        &mut self,
        identifier: String,
        loader: impl AssetLoader<A> + 'static,
    ) -> QPResult<AssetId> {
        let interner = self
            .resources
            .get_mut::<StringInterner>()
            .ok_or(QPError::ResourceNotFound("StringInterner".into()))?;

        let id = interner.intern(identifier.clone());

        let store = self
            .resources
            .get_mut::<AssetStore<A>>()
            .ok_or(QPError::ResourceNotFound(format!("AssetStore<{:?}>", id)))?;

        store.load_asset(loader, id)
    }

    pub fn unload_asset<A: Asset + 'static>(&mut self, id: AssetId) -> QPResult<()> {
        let store = self
            .resources
            .get_mut::<AssetStore<A>>()
            .ok_or(QPError::ResourceNotFound(format!("AssetStore<{:?}>", id)))?;

        store.unload_asset(id)
    }

    /// ///////////
    ///
    /// Cameras
    ///
    /// ///////////

    pub fn add_camera<C: Camera + 'static>(
        &mut self,
        identifier: String,
        camera: C,
    ) -> QPResult<CameraId> {
        let interner = self
            .resources
            .get_mut::<StringInterner>()
            .ok_or(QPError::ResourceNotFound("StringInterner".into()))?;

        let id = interner.intern(identifier.clone());

        let store = self
            .resources
            .get_mut::<CameraList>()
            .ok_or(QPError::ResourceNotFound("CameraList".into()))?;

        Ok(store.add_camera(id, camera))
    }

    /// ///////////
    ///
    /// Storage
    ///
    /// ///////////

    pub fn spawn(&mut self, bundle: impl Bundle) -> QPResult<Index> {
        let storage = self
            .resources
            .get_mut::<StorageManager>()
            .ok_or(QPError::ResourceNotFound("Storage Manager".into()))?
            .get_storage_mut(StorageId::Entities)
            .ok_or(QPError::Generic("Storage unit not found".into()))?;

        Ok(storage.create(bundle))
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
