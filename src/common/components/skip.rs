use crate::prelude::qp_ecs::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Component, Serialize, Deserialize, Clone, PartialEq)]
pub struct CSkip {}
