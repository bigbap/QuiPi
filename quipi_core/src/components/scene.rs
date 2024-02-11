use serde::{Deserialize, Serialize};

use crate::Component;

#[derive(Debug, Component, Serialize, Deserialize, PartialEq, Clone)]
pub struct CScene {
    pub name: String
}