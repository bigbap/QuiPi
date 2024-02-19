use serde::{Serialize, Deserialize};

use crate::{resources::{
    shader::UniformVariable, RShader
}, shaders::{SPRITE_FRAG, SPRITE_VERT}};

use super::ISchema;

pub const DEFAULT_SHADER_UNIFORM: &str = "mvpMatrix";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaSpriteShader {
    pub name: String,
    pub uniforms: Vec<UniformVariable>
}

impl Default for SchemaSpriteShader {
    fn default() -> Self {
        Self {
            name:  "sprite".to_string(),
            uniforms: vec![
                UniformVariable::MVPMatrix(DEFAULT_SHADER_UNIFORM.to_string())
            ],
        }
    }
}

impl ISchema for SchemaSpriteShader {
    fn load_resource(
        &self,
        registry: &mut crate::Registry
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let id = registry.load_resourse(
            self.name.clone(),
            RShader::from_str(SPRITE_VERT, SPRITE_FRAG, self.uniforms.to_vec())?
        )?;

        Ok(id)
    }

    fn from_resource(id: u64, registry: &crate::Registry) -> Option<Self> {
        if let (Some(shader), Some(name)) = (
            registry.get_resource::<RShader>(id),
            registry.string_interner.get_string(id)
         ) {
            let schema = SchemaSpriteShader {
                name,
                uniforms: shader.uniforms.clone()
            };

            return Some(schema);
        }

        println!("couldn't find shader: {}", id);

        None
    }
}
