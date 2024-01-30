use crate::Component;
use crate::wrappers::opengl::shader::ShaderProgram;

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

    pub fn program(&self) -> &ShaderProgram {
        &self.program
    }
}
