pub mod shader;
pub mod texture;

pub use shader::Shader;
pub use texture::Texture;

use crate::Registry;

pub fn register_resources(registry: &mut Registry) {
    registry
        .register_resource::<Shader>()
        .register_resource::<Texture>();
}
