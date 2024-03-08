use super::super::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Serialize, Deserialize, PartialEq, Clone)]
pub struct CDistance(pub f32);
