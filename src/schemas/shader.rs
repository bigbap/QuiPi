use serde::{Deserialize, Serialize};

use crate::prelude::qp_gfx::ShaderUniforms;
use crate::prelude::Schema;
use crate::prelude::{qp_assets::RShader, qp_gfx::get_shader, GlobalRegistry};
use crate::QPResult;

pub const DEFAULT_SHADER: &str = "sprite";
pub const DEFAULT_SHADER_UNIFORM: &str = "mvpMatrix";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaShader {
    pub name: String,
    pub uniforms: Vec<ShaderUniforms>,
}

impl Default for SchemaShader {
    fn default() -> Self {
        Self {
            name: DEFAULT_SHADER.to_string(),
            uniforms: vec![ShaderUniforms::MVPMatrix(
                DEFAULT_SHADER_UNIFORM.to_string(),
            )],
        }
    }
}

impl Schema for SchemaShader {
    fn load_resource(&self, registry: &mut GlobalRegistry) -> QPResult<u64> {
        let shader = get_shader(&self.name);
        let id = registry.asset_manager.load_asset(
            &self.name,
            RShader::from_str(shader.vert, shader.frag, self.uniforms.to_vec())?,
        )?;

        Ok(id)
    }

    fn from_resource(id: u64, registry: &GlobalRegistry) -> Option<Self> {
        if let (Some(shader), Some(name)) = (
            registry.asset_manager.get::<RShader>(id),
            registry.strings().get_string(id),
        ) {
            let schema = SchemaShader {
                name,
                uniforms: shader.uniforms.clone(),
            };

            return Some(schema);
        }

        println!("couldn't find shader: {}", id);

        None
    }
}
