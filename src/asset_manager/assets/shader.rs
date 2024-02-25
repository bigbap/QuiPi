use crate::QPResult;
use crate::platform::opengl::shader::ShaderProgram;
use crate::prelude::qp_data::ShaderUniforms;
use crate::prelude::qp_ecs::Component;

#[derive(Debug, Component, PartialEq)]
pub struct RShader {
    pub program: ShaderProgram,
    pub uniforms: Vec<ShaderUniforms>
}

impl RShader {
    pub fn new(
        file_name: &str,
        uniforms: Vec<ShaderUniforms>
    ) -> QPResult<Self> {
        Ok(Self {
            program: ShaderProgram::new(file_name)?,
            uniforms
        })
    }

    pub fn from_str(
        vert: &str,
        frag: &str,
        uniforms: Vec<ShaderUniforms>
    ) -> QPResult<Self> {
        Ok(Self {
            program: ShaderProgram::from_str(vert, frag)?,
            uniforms
        })
    }

    pub fn program(&self) -> &ShaderProgram {
        &self.program
    }
}
