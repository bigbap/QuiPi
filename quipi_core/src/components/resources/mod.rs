pub mod shader;
pub mod texture;

pub use shader::RShader;
pub use texture::RTexture;

use crate::Registry;

use super::CName;

pub fn register_resources(registry: &mut Registry) {
    registry.resources
        .register_component::<RShader>()
        .register_component::<RTexture>();

    registry.resources.register_component::<CName>();
}
