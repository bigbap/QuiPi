use quipi_core::{
    Component,
    VersionedIndex
};

#[derive(Debug, Component, PartialEq)]
pub struct CSprite {
    pub shader: VersionedIndex,
    pub camera: VersionedIndex,
    pub texture: VersionedIndex,
}
