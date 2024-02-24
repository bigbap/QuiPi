use serde::{Serialize, Deserialize};

use crate::prelude::{
    qp_ecs::resources::RShader,
    qp_data::{
        ShaderUniforms,
        ISchema
    },
    qp_gfx::get_shader,
    Registry
};

pub const DEFAULT_SHADER: &str = "sprite";
pub const DEFAULT_SHADER_UNIFORM: &str = "mvpMatrix";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaShader {
    pub name: String,
    pub uniforms: Vec<ShaderUniforms>
}

impl Default for SchemaShader {
    fn default() -> Self {
        Self {
            name: DEFAULT_SHADER.to_string(),
            uniforms: vec![
                ShaderUniforms::MVPMatrix(DEFAULT_SHADER_UNIFORM.to_string())
            ],
        }
    }
}

impl ISchema for SchemaShader {
    fn load_resource(
        &self,
        registry: &mut Registry
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let shader = get_shader(&self.name);
        let id = registry.load_resourse(
            self.name.to_string(),
            RShader::from_str(shader.vert, shader.frag, self.uniforms.to_vec())?
        )?;

        Ok(id)
    }

    fn from_resource(id: u64, registry: &Registry) -> Option<Self> {
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
