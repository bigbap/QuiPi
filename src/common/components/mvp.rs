use crate::prelude::qp_storage::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Component, Serialize, Deserialize, PartialEq, Clone)]
pub struct CModelMatrix(pub glm::Mat4);

#[derive(Debug, Default, Component, Serialize, Deserialize, PartialEq, Clone)]
pub struct CViewMatrix(pub glm::Mat4);

#[derive(Debug, Default, Component, Serialize, Deserialize, PartialEq, Clone)]
pub struct CProjectionMatrix(pub glm::Mat4);

#[derive(Debug, Default, Component, Serialize, Deserialize, PartialEq, Clone)]
pub struct CMVPMatrix(pub glm::Mat4);
