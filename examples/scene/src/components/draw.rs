use engine::{
    Component,
    VersionedIndex
};

#[derive(Debug, Default, Component, PartialEq)]
pub struct DrawComponent {
    pub shader_id: VersionedIndex
}

