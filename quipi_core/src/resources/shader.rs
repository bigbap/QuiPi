use serde::{Serialize, Deserialize};

use crate::Component;
use crate::platform::opengl::shader::ShaderProgram;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UniformVariable {
    MVPMatrix(String),
    ModelMatrix(String),
    ViewMatrix(String),
    ProjectionMatrix(String),
    Color(String),
    NearPlane(String),
    FarPlane(String),
}

#[derive(Debug, Component, PartialEq)]
pub struct RShader {
    pub program: ShaderProgram,
    pub uniforms: Vec<UniformVariable>
}

impl RShader {
    pub fn new(
        file_name: &str,
        uniforms: Vec<UniformVariable>
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            program: ShaderProgram::new(file_name)?,
            uniforms
        })
    }

    pub fn from_str(
        vert: &str,
        frag: &str,
        uniforms: Vec<UniformVariable>
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            program: ShaderProgram::from_str(vert, frag)?,
            uniforms
        })
    }

    pub fn program(&self) -> &ShaderProgram {
        &self.program
    }
}
