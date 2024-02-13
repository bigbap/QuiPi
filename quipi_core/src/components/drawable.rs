use crate::{
    core::rendering::mesh::ElementArray, Component, VersionedIndex
};

#[derive(Debug, Component, PartialEq)]
pub struct CDrawable {
    pub shader: VersionedIndex,
    pub camera: VersionedIndex,
}

#[derive(Debug, Component, PartialEq)]
pub struct CElementArray(pub ElementArray);
