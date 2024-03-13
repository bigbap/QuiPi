use super::{cameras::CRenderLayer, Mesh};
use crate::{
    assets::AssetHandle,
    prelude::{QPError, World},
    storage::prelude::{Component, ComponentId, Index, StorageId, StorageManager},
    QPResult,
};
use std::collections::HashMap;

pub fn start_render_pipeline(world: &mut World) -> QPResult<()> {
    let data = HashMap::<CRenderLayer, Vec<(Index, CMeshId)>>::new();

    let storage = world
        .resources
        .get::<StorageManager>()
        .ok_or(QPError::ResourceNotFound("Storage Manager".into()))?;
    let render_layers = storage.query::<CRenderLayer>(StorageId::Cameras);

    Ok(())
}

#[derive(Component, PartialEq, Clone)]
pub struct CMeshId {
    pub mesh: AssetHandle<Mesh>,
}
