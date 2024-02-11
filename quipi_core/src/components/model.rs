use crate::systems::assets::ObjectConfig;
use crate::systems::rendering::mesh::ElementArrayMesh;
use crate::{Component, VersionedIndex};

// clean this component
#[derive(Debug, Default, Component, PartialEq)]
pub struct CModelNode {
    pub mesh: Option<ElementArrayMesh>,
    pub data: ObjectConfig,

    pub shader: String,
    pub children: Option<Vec<VersionedIndex>>
}
