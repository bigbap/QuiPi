use serde::{Serialize, Deserialize};

use crate::Component;

#[derive(Debug, Component, Clone, Serialize, Deserialize, PartialEq)]
pub struct CTag {
    pub tag: String
}

#[derive(Debug, Component, Clone, Serialize, Deserialize, PartialEq)]
pub struct CName {
    pub name: String
}