pub use macros::Resource;

use std::collections::HashMap;

use crate::{
    common::resources::{
        Asset, AssetId, AssetLoader, AssetStore, Camera, CameraId, CameraList, StringInterner,
    },
    prelude::QPError,
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

    // /// ///////////
    // ///
    // /// Assets
    // ///
    // /// ///////////

    // pub fn load_asset<A: Asset + 'static>(
    //     &mut self,
    //     identifier: &str,
    //     loader: impl AssetLoader<A>,
    // ) -> QPResult<AssetId> {
    //     let interner = self
    //         .get_mut::<StringInterner>()
    //         .ok_or(QPError::ResourceNotFound("StringInterner".into()))?;

    //     let id = interner.intern(identifier.into());

    //     let store = self
    //         .get_mut::<AssetStore<A>>()
    //         .ok_or(QPError::ResourceNotFound(format!("AssetStore<{:?}>", id)))?;

    //     store.load_asset(loader, id)
    // }

    // pub fn unload_asset<A: Asset + 'static>(&mut self, id: AssetId) -> QPResult<()> {
    //     let store = self
    //         .get_mut::<AssetStore<A>>()
    //         .ok_or(QPError::ResourceNotFound(format!("AssetStore<{:?}>", id)))?;

    //     store.unload_asset(id)
    // }

    // pub fn get_asset<A: Asset + 'static>(&self, id: &AssetId) -> Option<&A> {
    //     self.get::<AssetStore<A>>()?.get(id)
    // }

    // pub fn get_asset_mut<A: Asset + 'static>(&mut self, id: &AssetId) -> Option<&mut A> {
    //     self.get_mut::<AssetStore<A>>()?.get_mut(id)
    // }

    // pub fn get_asset_id<A: Asset + 'static>(&mut self, identifier: &str) -> QPResult<AssetId> {
    //     let interner = self
    //         .get_mut::<StringInterner>()
    //         .ok_or(QPError::ResourceNotFound("StringInterner".into()))?;

    //     let id = interner.intern(identifier.into());

    //     let store = self
    //         .get::<AssetStore<A>>()
    //         .ok_or(QPError::ResourceNotFound(format!("AssetStore<{:?}>", id)))?;

    //     Ok(store.asset_id(id))
    // }

    // /// ///////////
    // ///
    // /// Cameras
    // ///
    // /// ///////////

    // pub fn add_camera<C: Camera + 'static>(
    //     &mut self,
    //     identifier: &str,
    //     camera: C,
    // ) -> QPResult<CameraId> {
    //     let interner = self
    //         .get_mut::<StringInterner>()
    //         .ok_or(QPError::ResourceNotFound("StringInterner".into()))?;

    //     let id = interner.intern(identifier.into());

    //     let store = self
    //         .get_mut::<CameraList>()
    //         .ok_or(QPError::ResourceNotFound("CameraList".into()))?;

    //     Ok(store.add_camera(id, camera))
    // }

    // pub fn get_camera<C: Camera + 'static>(&self, id: &CameraId) -> Option<&C> {
    //     self.get::<CameraList>()?.get(id)
    // }

    // pub fn get_camera_mut<C: Camera + 'static>(&mut self, id: &CameraId) -> Option<&mut C> {
    //     self.get_mut::<CameraList>()?.get_mut(id)
    // }
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
