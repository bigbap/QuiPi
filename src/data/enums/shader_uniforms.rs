use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ShaderUniforms {
    MVPMatrix(String),
    ModelMatrix(String),
    ViewMatrix(String),
    ProjectionMatrix(String),
    Color(String),
    NearPlane(String),
    FarPlane(String),
}