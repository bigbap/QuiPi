use serde::{Serialize, Deserialize};

use crate::{Component, Registry};

#[derive(Debug, Component, Clone, Serialize, Deserialize, PartialEq)]
pub struct CTag {
    pub tag: String
}

#[derive(Debug, Component, Clone, Serialize, Deserialize, PartialEq)]
pub struct CName {
    name: String
}
 
impl CName {
    pub fn new(name: &str, registry: &Registry) -> Self {
        // TODO: validate that this is unique

        Self {
            name: name.to_string()
        }
    }

    pub fn get(&self) -> String {
        self.name.clone()
    }
}
