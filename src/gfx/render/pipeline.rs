use super::{cameras::CRenderLayer, Mesh};
use crate::{
    assets::AssetHandle,
    prelude::{Component, Index, QPError, StorageId, StorageManager, World},
    QPResult,
};
use std::collections::HashMap;

pub fn start_render_pipeline(world: &mut World) {
    let mut data_acc = HashMap::<CRenderLayer, Vec<(Index, CMeshId)>>::new();

    // get camera render layers
    let Some(render_layers) = world.entity_iter::<CRenderLayer>(StorageId::Cameras) else {
        #[cfg(debug_assertions)]
        println!("Couldn't get camera render layers");

        return;
    };

    // get entity render layers
    let mut entity_layers = Vec::<(Index, &CRenderLayer)>::new();
    let Some(render_layers_entities) = world.entity_iter::<CRenderLayer>(StorageId::Entities)
    else {
        #[cfg(debug_assertions)]
        println!("Couldn't get entity render layers");

        return;
    };
    for item in render_layers_entities {
        if let Some((index, layer)) = item {
            entity_layers.push((index, layer));
        }
    }

    // get camera render layers
    for layer in render_layers {
        let Some((_, layer)) = layer else {
            continue;
        };

        let data = data_acc
            .get_mut(&layer)
            .get_or_insert(&mut Vec::<(Index, CMeshId)>::new());
    }
}

#[derive(Component, PartialEq, Clone)]
pub struct CMeshId {
    pub mesh: AssetHandle<Mesh>,
}
