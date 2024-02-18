pub mod shader;
pub mod texture;

pub use shader::RShader;
pub use texture::RTexture;

use crate::Registry;

pub fn register_resources(registry: &mut Registry) {
    registry
        .register_resource::<RShader>()
        .register_resource::<RTexture>();
}
