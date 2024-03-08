use super::super::prelude::{Component, Index};

#[derive(Component, Debug, PartialEq)]
pub struct CChildren {
    pub list: Vec<Index>,
}
