use super::{cameras::CRenderLayer, Mesh};
use crate::{
    assets::AssetHandle,
    prelude::{Component, Index, QPError, StorageId, StorageManager, World},
    QPResult,
};
use std::collections::HashMap;

pub fn start_render_pipeline(world: &mut World) -> QPResult<()> {
    let data = HashMap::<CRenderLayer, Vec<(Index, CMeshId)>>::new();

    let Some(storage) = world.storage().get(StorageId::Cameras) else {
        println!("Couldn't get camera storage");

        return Ok(());
    };

    let render_layers = storage.query::<CRenderLayer>();

    Ok(())
}

#[derive(Component, PartialEq, Clone)]
pub struct CMeshId {
    pub mesh: AssetHandle<Mesh>,
}
