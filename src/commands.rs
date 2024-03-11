use crate::{
    common::resources::{
        Asset, AssetId, AssetLoader, AssetStore, Camera, CameraId, CameraList, StringInterner,
    },
    prelude::QPError,
    resources::{Resource, ResourceId, ResourceManager},
    QPResult,
};

pub struct Commands {
    resources: &'static mut ResourceManager,
}

impl Commands {
    pub(crate) fn new(resources: &'static mut ResourceManager) -> Self {
        Self { resources }
    }

    pub fn add_resource<R: Resource + 'static>(&mut self, resource: R) -> QPResult<ResourceId> {
        self.resources.add_resource::<R>(resource)
    }

    /// ///////////
    ///
    /// Assets
    ///
    /// ///////////

    pub fn load_asset<A: Asset + 'static>(
        &mut self,
        identifier: &str,
        loader: impl AssetLoader<A>,
    ) -> QPResult<AssetId> {
        let interner = self
            .resources
            .get_mut::<StringInterner>()
            .ok_or(QPError::ResourceNotFound("StringInterner".into()))?;

        let id = interner.intern(identifier.into());

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
        identifier: &str,
        camera: C,
    ) -> QPResult<CameraId> {
        let interner = self
            .resources
            .get_mut::<StringInterner>()
            .ok_or(QPError::ResourceNotFound("StringInterner".into()))?;

        let id = interner.intern(identifier.into());

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

    pub fn spawn(&mut self) {}
}
