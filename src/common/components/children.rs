use crate::prelude::qp_ecs::*;

#[derive(Component, Debug, PartialEq)]
pub struct CChildren {
    pub list: Vec<Index>,
}
