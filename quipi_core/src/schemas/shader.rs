use serde::{Serialize, Deserialize};

use crate::resources::{
    RShader,
    shader::UniformVariable
};

use super::ISchema;


pub const DEFAULT_SHADER: &str = "default";
pub const DEFAULT_SHADER_UNIFORM: &str = "mvpMatrix";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaShader {
    pub name: String,
    pub uniforms: Vec<UniformVariable>
}

impl Default for SchemaShader {
    fn default() -> Self {
        Self {
            name:  DEFAULT_SHADER.to_string(),
            uniforms: vec![
                UniformVariable::MVPMatrix(DEFAULT_SHADER_UNIFORM.to_string())
            ],
        }
    }
}

impl ISchema for SchemaShader {
    fn load_resource(
        &self,
        registry: &mut crate::Registry
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let id = registry.load_resourse(
            self.name.clone(),
            RShader::new(&self.name, self.uniforms.to_vec())?
        )?;

        Ok(id)
    }

    fn from_resource(id: u64, registry: &crate::Registry) -> Option<Self> {
        if let (Some(shader), Some(name)) = (
            registry.get_resource::<RShader>(id),
            registry.string_interner.get_string(id)
         ) {
            let schema = SchemaShader {
                name,
                uniforms: shader.uniforms.clone()
            };

            return Some(schema);
        }

        println!("couldn't find shader: {}", id);

        None
    }
}
