use serde::{Serialize, Deserialize};

use crate::Component;

#[derive(Debug, Component, PartialEq, Clone, Serialize, Deserialize)]
pub struct CDistance(pub f32);
