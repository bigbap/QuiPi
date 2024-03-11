use crate::{
    common::resources::{Asset, AssetId, AssetStore, Camera, CameraId, CameraList, StringInterner},
    prelude::QPError,
    resources::{Resource, ResourceManager},
    storage::prelude::{Component, Index, IndexedArray, StorageId::*, StorageManager},
    QPResult,
};

pub struct Query {
    resources: &'static mut ResourceManager,
}

impl Query {
    pub(crate) fn new(resources: &'static mut ResourceManager) -> Self {
        Self { resources }
    }

    /// ///////////
    ///
    /// Resources
    ///
    /// ///////////

    pub fn resource<R: Resource + 'static>(&self) -> Option<&R> {
        self.resources.get::<R>()
    }

    pub fn resource_mut<R: Resource + 'static>(&mut self) -> Option<&mut R> {
        self.resources.get_mut::<R>()
    }

    /// ///////////
    ///
    /// Assets
    ///
    /// ///////////

    pub fn asset<A: Asset + 'static>(&self, id: &AssetId) -> Option<&A> {
        self.resource::<AssetStore<A>>()?.get(id)
    }

    pub fn asset_mut<A: Asset + 'static>(&mut self, id: &AssetId) -> Option<&mut A> {
        self.resource_mut::<AssetStore<A>>()?.get_mut(id)
    }

    pub fn asset_id<A: Asset + 'static>(&mut self, identifier: &str) -> QPResult<AssetId> {
        let interner = self
            .resource_mut::<StringInterner>()
            .ok_or(QPError::ResourceNotFound("StringInterner".into()))?;

        let id = interner.intern(identifier.into());

        let store = self
            .resource::<AssetStore<A>>()
            .ok_or(QPError::ResourceNotFound(format!("AssetStore<{:?}>", id)))?;

        Ok(store.asset_id(id))
    }

    /// ///////////
    ///
    /// Cameras
    ///
    /// ///////////

    pub fn camera<C: Camera + 'static>(&self, id: &CameraId) -> Option<&C> {
        self.resource::<CameraList>()?.get(id)
    }

    pub fn camera_mut<C: Camera + 'static>(&mut self, id: &CameraId) -> Option<&mut C> {
        self.resource_mut::<CameraList>()?.get_mut(id)
    }

    /// ///////////
    ///
    /// Entities
    ///
    /// ///////////

    pub fn entity<C: Component + PartialEq + 'static>(&self, index: &Index) -> Option<&C> {
        self.resource::<StorageManager>()?.get::<C>(Entities, index)
    }

    pub fn entity_mut<C: Component + PartialEq + 'static>(
        &mut self,
        index: &Index,
    ) -> Option<&mut C> {
        self.resource_mut::<StorageManager>()?
            .get_mut::<C>(Entities, index)
    }

    pub fn entities<C: Component + PartialEq + 'static>(&self) -> Option<&IndexedArray<C>> {
        self.resource::<StorageManager>()?.query::<C>(Entities)
    }

    pub fn entities_mut<C: Component + PartialEq + 'static>(
        &mut self,
    ) -> Option<&mut IndexedArray<C>> {
        self.resource_mut::<StorageManager>()?
            .query_mut::<C>(Entities)
    }
}
