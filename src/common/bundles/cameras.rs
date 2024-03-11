use crate::prelude::qp_common::components::*;
use crate::prelude::qp_ecs::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Component, Serialize, Deserialize, PartialEq, Clone)]
pub struct CCamera2D {}

pub fn camera_2d_bundle() -> impl Bundle {
    (
        CCamera2D::default(),
        CTransform2D::default(),
        COrthographic::default(),
        CViewMatrix::default(),
        CProjectionMatrix::default(),
    )
}

pub fn camera_3d_bundle() -> impl Bundle {
    todo!()
}
