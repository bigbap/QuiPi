use crate::Component;
use serde::{Serialize, Deserialize};


#[derive(Debug, Component, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct CSpeed(f32);
