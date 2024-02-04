use serde::{Serialize, Deserialize};

use crate::{
    resources::{
        Shader,
        shader::UniformVariable
    },
    VersionedIndex
};

use super::{ISchema, SchemaError};


pub const DEFAULT_SHADER: &str = "default";
pub const DEFAULT_SHADER_UNIFORM: &str = "mvpMatrix";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaShader {
    name: String,
    uniforms: Vec<UniformVariable>
}

impl Default for SchemaShader {
    fn default() -> Self {
        Self {
            name: DEFAULT_SHADER.to_string(),
            uniforms: vec![
                UniformVariable::MVPMatrix(DEFAULT_SHADER_UNIFORM.to_string())
            ],
        }
    }
}

impl ISchema for SchemaShader {
    fn build(
        &self,
        registry: &mut crate::Registry
    ) -> Result<VersionedIndex, SchemaError> {
        Ok(
            registry.create_resource(
                &self.name,
                Shader::new(
                    &self.name,
                    self.uniforms.to_vec()
                )?
            )?
        )
    }
}
