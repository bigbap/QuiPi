use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ValidTile<T> {
    Invalid,
    Valid(T)
}
