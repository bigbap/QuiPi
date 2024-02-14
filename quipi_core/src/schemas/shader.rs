use serde::{Serialize, Deserialize};

use crate::{
    resources::{
        RShader,
        shader::UniformVariable
    },
    VersionedIndex, components::CName
};

use super::ISchema;


pub const DEFAULT_SHADER: &str = "default";
pub const DEFAULT_SHADER_UNIFORM: &str = "mvpMatrix";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaShader {
    pub name: CName,
    pub uniforms: Vec<UniformVariable>
}

impl Default for SchemaShader {
    fn default() -> Self {
        Self {
            name: CName { name: DEFAULT_SHADER.to_string() },
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
    ) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
        let res = registry.resources.create();
        registry.resources.add(&res, self.name.clone());
        registry.resources.add(&res, RShader::new(&self.name.name, self.uniforms.to_vec())?);

        Ok(res)
    }

    fn from_entity(entity: VersionedIndex, registry: &crate::Registry) -> Option<Self> {
        if let (Some(name), Some(shader)) = (
            registry.resources.get::<CName>(&entity),
            registry.resources.get::<RShader>(&entity)
        ) {
            let schema = SchemaShader {
                name: name.clone(),
                uniforms: shader.uniforms.clone()
            };

            return Some(schema);
        }
        None
    }
}
