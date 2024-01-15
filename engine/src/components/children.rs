use crate::{
    Component,
    VersionedIndex
};

#[derive(Debug, Component)]
pub struct Children {
    pub children: Vec<VersionedIndex>
}
