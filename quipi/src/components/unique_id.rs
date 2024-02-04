use serde::{Serialize, Deserialize};

use crate::Component;


#[derive(Debug, Component, Serialize, Deserialize, Clone)]
pub struct CUniqueId(pub String);
