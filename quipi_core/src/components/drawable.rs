use crate::{
    Component,
    VersionedIndex
};

#[derive(Debug, Component, PartialEq)]
pub struct CDrawable {
    pub shader: VersionedIndex,
    pub camera: VersionedIndex,
}
