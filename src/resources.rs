pub use macros::Resource;

use std::collections::HashMap;

use crate::{
    common::resources::{Asset, AssetId, AssetStore, Camera, CameraId, CameraList, StringInterner},
    prelude::QPError,
    storage::prelude::{Component, Index, IndexedArray, StorageId::*, StorageManager},
    QPResult,
};

pub struct ResourceManager {
    resources: HashMap<ResourceId, Box<dyn Resource + 'static>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn add_resource<R: Resource + 'static>(&mut self, resource: R) -> QPResult<ResourceId> {
        let id = ResourceId::new::<R>();
        if self.resources.get(&id).is_some() {
            return Err(QPError::DuplicateResource);
        }

        self.resources.insert(id, Box::new(resource));

        Ok(id)
    }

    pub fn get<R: Resource + 'static>(&self) -> Option<&R> {
        match self.resources.get(&ResourceId::new::<R>()) {
            Some(resource) => resource.as_any().downcast_ref::<R>(),
            _ => None,
        }
    }

    pub fn get_mut<R: Resource + 'static>(&mut self) -> Option<&mut R> {
        match self.resources.get_mut(&ResourceId::new::<R>()) {
            Some(resource) => resource.as_any_mut().downcast_mut::<R>(),
            _ => None,
        }
    }

    /// ///////////
    ///
    /// Assets
    ///
    /// ///////////

    pub fn asset<A: Asset + 'static>(&self, id: &AssetId) -> Option<&A> {
        self.get::<AssetStore<A>>()?.get(id)
    }

    pub fn asset_mut<A: Asset + 'static>(&mut self, id: &AssetId) -> Option<&mut A> {
        self.get_mut::<AssetStore<A>>()?.get_mut(id)
    }

    pub fn asset_id<A: Asset + 'static>(&mut self, identifier: &str) -> QPResult<AssetId> {
        let interner = self
            .get_mut::<StringInterner>()
            .ok_or(QPError::ResourceNotFound("StringInterner".into()))?;

        let id = interner.intern(identifier.into());

        let store = self
            .get::<AssetStore<A>>()
            .ok_or(QPError::ResourceNotFound(format!("AssetStore<{:?}>", id)))?;

        Ok(store.asset_id(id))
    }

    /// ///////////
    ///
    /// Cameras
    ///
    /// ///////////

    pub fn camera<C: Camera + 'static>(&self, id: &CameraId) -> Option<&C> {
        self.get::<CameraList>()?.get(id)
    }

    pub fn camera_mut<C: Camera + 'static>(&mut self, id: &CameraId) -> Option<&mut C> {
        self.get_mut::<CameraList>()?.get_mut(id)
    }

    /// ///////////
    ///
    /// Entities
    ///
    /// ///////////

    pub fn entity<C: Component + PartialEq + 'static>(&self, index: &Index) -> Option<&C> {
        self.get::<StorageManager>()?.get::<C>(Entities, index)
    }

    pub fn entity_mut<C: Component + PartialEq + 'static>(
        &mut self,
        index: &Index,
    ) -> Option<&mut C> {
        self.get_mut::<StorageManager>()?
            .get_mut::<C>(Entities, index)
    }

    pub fn entities<C: Component + PartialEq + 'static>(&self) -> Option<&IndexedArray<C>> {
        self.get::<StorageManager>()?.query::<C>(Entities)
    }

    pub fn entities_mut<C: Component + PartialEq + 'static>(
        &mut self,
    ) -> Option<&mut IndexedArray<C>> {
        self.get_mut::<StorageManager>()?.query_mut::<C>(Entities)
    }
}

pub trait Resource: AsAny {
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ResourceId(std::any::TypeId);

impl ResourceId {
    pub fn new<R: Resource + 'static>() -> Self {
        ResourceId(std::any::TypeId::of::<R>())
    }
}

pub use crate::core::prelude::AsAny;
