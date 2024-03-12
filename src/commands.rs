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
}
