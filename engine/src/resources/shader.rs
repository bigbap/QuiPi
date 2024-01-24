use crate::Component;
use crate::core::ShaderProgram;

#[derive(Debug)]
pub enum UniformVariable {
    MVPMatrix(String),
    ModelMatrix(String),
    ViewMatrix(String),
    ProjectionMatrix(String),
    Color(String),
    NearPlane(String),
    FarPlane(String),
}

#[derive(Debug, Component)]
pub struct Shader {
    pub program: ShaderProgram,
    pub uniforms: Vec<UniformVariable>
}

impl Shader {
    pub fn new(
        file_name: &str,
        uniforms: Vec<UniformVariable>
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            program: ShaderProgram::new(file_name)?,
            uniforms
        })
    }

    pub fn program(&self) -> &ShaderProgram {
        &self.program
    }
}
