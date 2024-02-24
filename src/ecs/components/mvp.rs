use serde::{Deserialize, Serialize};

use super::super::prelude::Component;


#[derive(Debug, Component, Serialize, Deserialize, PartialEq)]
pub struct CModelMatrix(pub glm::Mat4);

#[derive(Debug, Component, Serialize, Deserialize, PartialEq)]
pub struct CViewMatrix(pub glm::Mat4);

#[derive(Debug, Component, Serialize, Deserialize, PartialEq)]
pub struct CProjectionMatrix(pub glm::Mat4);

#[derive(Debug, Component, Serialize, Deserialize, PartialEq)]
pub struct CMVPMatrix(pub glm::Mat4);