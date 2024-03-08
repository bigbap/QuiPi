use serde::{Deserialize, Serialize};

use super::super::prelude::*;

#[derive(Debug, Component, Clone, Serialize, Deserialize, PartialEq)]
pub struct CTag {
    pub tag: String,
}

#[derive(Debug, Component, Clone, Serialize, Deserialize, PartialEq)]
pub struct CName {
    pub name: String,
}
