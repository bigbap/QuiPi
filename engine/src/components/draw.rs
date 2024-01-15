use crate::{
    Component,
    VersionedIndex,
    gfx::Material,
};

#[derive(Debug, Default, Component)]
pub struct Draw {
    pub shader_id: VersionedIndex,
    pub camera_id: VersionedIndex,
    pub materials: Vec<Material>,
}
