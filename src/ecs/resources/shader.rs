use super::super::api::Component;
use crate::platform::opengl::shader::ShaderProgram;
use crate::prelude::data::ShaderUniforms;

#[derive(Debug, Component, PartialEq)]
pub struct RShader {
    pub program: ShaderProgram,
    pub uniforms: Vec<ShaderUniforms>
}

impl RShader {
    pub fn new(
        file_name: &str,
        uniforms: Vec<ShaderUniforms>
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            program: ShaderProgram::new(file_name)?,
            uniforms
        })
    }

    pub fn from_str(
        vert: &str,
        frag: &str,
        uniforms: Vec<ShaderUniforms>
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
