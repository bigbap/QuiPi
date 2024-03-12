use crate::{
    common::resources::{
        Asset, AssetId, AssetLoader, AssetStore, Camera, CameraId, CameraList, StringInterner,
    },
    prelude::{QPError, World},
    resources::{Resource, ResourceId},
    storage::prelude::{Bundle, Index, StorageId, StorageManager},
    QPResult,
};

pub struct Commands<'w> {
    world: &'w mut World,
}

impl<'w> Commands<'w> {
    pub(crate) fn new(world: &'w mut World) -> Self {
        Self { world }
    }

    pub fn add_resource<R: Resource + Copy + 'static>(
        &mut self,
        resource: R,
    ) -> QPResult<ResourceId> {
        self.world.resources.add_resource::<R>(resource)
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
            .world
            .resources
            .get_mut::<StringInterner>()
            .ok_or(QPError::ResourceNotFound("StringInterner".into()))?;

        let id = interner.intern(identifier.clone());

        let store = self
            .world
            .resources
            .get_mut::<AssetStore<A>>()
            .ok_or(QPError::ResourceNotFound(format!("AssetStore<{:?}>", id)))?;

        store.load_asset(loader, id)
    }

    pub fn unload_asset<A: Asset + 'static>(&mut self, id: AssetId) -> QPResult<()> {
        let store = self
            .world
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
            .world
            .resources
            .get_mut::<StringInterner>()
            .ok_or(QPError::ResourceNotFound("StringInterner".into()))?;

        let id = interner.intern(identifier.clone());

        let store = self
            .world
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
            .world
            .resources
            .get_mut::<StorageManager>()
            .ok_or(QPError::ResourceNotFound("Storage Manager".into()))?
            .get_storage_mut(StorageId::Entities)
            .ok_or(QPError::Generic("Storage unit not found".into()))?;

        Ok(storage.create(bundle))
    }
}
