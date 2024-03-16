use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Serialize, Deserialize, PartialEq, Clone)]
pub struct CScene {
    pub id: u64,
    pub cameras: Vec<u64>,
    pub shaders: Vec<u64>,
    pub textures: Vec<u64>,
}
