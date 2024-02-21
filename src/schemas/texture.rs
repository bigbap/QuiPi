use crate::{
    opengl::textures::{
        ParameterName,
        ParameterValue
    },
    core::rendering::texture::from_image,
    core::utils::to_abs_path,
    modules::ecs::resources::RTexture
};
use serde::{Serialize, Deserialize};

use super::ISchema;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaTexture {
    pub name: String,
    pub texture_dims: glm::Vec2
}

impl ISchema for SchemaTexture {
    fn load_resource(
        &self,
        registry: &mut crate::Registry
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let path = format!("assets/textures/{}", self.name);

        let texture = from_image(&to_abs_path(&path)?)?;
        texture
            .set_parameter(ParameterName::WrapS, ParameterValue::ClampToEdge)
            .set_parameter(ParameterName::WrapT, ParameterValue::ClampToEdge)
            .set_parameter(ParameterName::MinFilter, ParameterValue::LinearMipmapNearest)
            .set_parameter(ParameterName::MagFilter, ParameterValue::Nearest);

        let id = registry.load_resourse(self.name.clone(), RTexture {
            texture,
            texture_dims: self.texture_dims
        })?;

        Ok(id)
    }

    fn from_resource(id: u64, registry: &crate::Registry) -> Option<Self> {
        if let (Some(texture), Some(name)) = (
            registry.get_resource::<RTexture>(id),
            registry.string_interner.get_string(id)
         ) {
            let schema = SchemaTexture {
                name,
                texture_dims: texture.texture_dims
            };

            return Some(schema);
        }

        println!("couldn't find texture: {}", id);

        None
    }
}
