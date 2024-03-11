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
}
