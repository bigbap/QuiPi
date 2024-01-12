use engine::{
    Component,
    VersionedIndex,
};

#[derive(Debug, Default, Component)]
pub struct DrawComponent {
    pub shader_id: VersionedIndex,
    pub textures: Vec<VersionedIndex>
}
