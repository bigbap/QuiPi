use crate::{
    Component,
    VersionedIndex
};

#[derive(Debug, Component, Clone, PartialEq)]
pub struct CTexture(pub VersionedIndex);