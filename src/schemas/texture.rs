use crate::{
    platform::opengl::textures::{
        ParameterName,
        ParameterValue
    },
    prelude::{
        gfx::texture::from_image,
        core::to_abs_path,
        ecs::resources::RTexture,
        data::ISchema,
        Registry
    },
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaTexture {
    pub name: String,
    pub texture_dims: glm::Vec2
}

impl ISchema for SchemaTexture {
    fn load_resource(
        &self,
        registry: &mut Registry
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

    fn from_resource(id: u64, registry: &Registry) -> Option<Self> {
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
