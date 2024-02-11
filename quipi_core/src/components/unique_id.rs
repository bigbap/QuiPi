use serde::{Serialize, Deserialize};

use crate::Component;


#[derive(Debug, Component, Serialize, Deserialize, Clone, PartialEq)]
pub struct CUniqueId(pub String);
