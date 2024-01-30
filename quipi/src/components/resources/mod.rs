pub mod shader;
pub mod texture;

pub use shader::RShader as Shader;
pub use texture::RTexture as Texture;

use crate::Registry;

pub fn register_resources(registry: &mut Registry) {
    registry
        .register_resource::<Shader>()
        .register_resource::<Texture>();
}
