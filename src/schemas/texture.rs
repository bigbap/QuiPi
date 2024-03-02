use crate::{
    platform::opengl::textures::{ParameterName, ParameterValue},
    prelude::{
        qp_assets::RTexture, qp_core::to_abs_path, qp_gfx::texture::from_image, GlobalRegistry,
        Schema,
    },
    QPResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaTexture {
    pub name: String,
    pub texture_dims: glm::Vec2,
}

impl Schema for SchemaTexture {
    fn load_resource(&self, registry: &mut GlobalRegistry) -> QPResult<u64> {
        let path = format!("assets/textures/{}", self.name);

        let texture = from_image(&to_abs_path(&path)?)?;
        texture
            .set_parameter(ParameterName::WrapS, ParameterValue::ClampToEdge)
            .set_parameter(ParameterName::WrapT, ParameterValue::ClampToEdge)
            .set_parameter(ParameterName::MinFilter, ParameterValue::Linear)
            .set_parameter(ParameterName::MagFilter, ParameterValue::Nearest);

        let id = registry.asset_manager.load_asset(
            &self.name,
            RTexture {
                texture,
                texture_dims: self.texture_dims,
            },
        )?;

        Ok(id)
    }

    fn from_resource(id: u64, registry: &GlobalRegistry) -> Option<Self> {
        if let (Some(texture), Some(name)) = (
            registry.asset_manager.get::<RTexture>(id),
            registry.strings().get_string(id),
        ) {
            let schema = SchemaTexture {
                name,
                texture_dims: texture.texture_dims,
            };

            return Some(schema);
        }

        println!("couldn't find texture: {}", id);

        None
    }
}
