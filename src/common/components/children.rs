use crate::prelude::qp_storage::*;

#[derive(Component, Debug, PartialEq, Clone)]
pub struct CChildren {
    pub list: Vec<Index>,
}
