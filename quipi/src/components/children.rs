use crate::{Component, VersionedIndex};

#[derive(Component, Debug)]
pub struct CChildren {
    pub list: Vec<VersionedIndex>
}
