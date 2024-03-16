use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Serialize, Deserialize, Clone, PartialEq)]
pub struct CColor(pub f32, pub f32, pub f32, pub f32);
