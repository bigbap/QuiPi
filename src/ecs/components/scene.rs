use serde::{Deserialize, Serialize};

use super::super::prelude::*;

#[derive(Debug, Component, Serialize, Deserialize, PartialEq, Clone)]
pub struct CScene {
    pub id: u64,
    pub cameras: Vec<u64>,
    pub shaders: Vec<u64>,
    pub textures: Vec<u64>,
}
