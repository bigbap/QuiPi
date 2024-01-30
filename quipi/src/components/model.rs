use crate::systems::rendering::mesh::ElementArrayMesh;
use crate::{Component, VersionedIndex};

#[derive(Debug, Default, Component)]
pub struct CModelNode {
    pub mesh: Option<ElementArrayMesh>,
    pub children: Vec<VersionedIndex>
}
