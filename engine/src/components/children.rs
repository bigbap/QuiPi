use crate::{
    Component,
    VersionedIndex
};

#[derive(Debug, Component)]
pub struct ChildrenComponent {
    pub entities: Vec<VersionedIndex>
}
