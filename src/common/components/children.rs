use crate::prelude::qp_ecs::*;

#[derive(Component, Debug, PartialEq, Clone)]
pub struct CChildren {
    pub list: Vec<Index>,
}
