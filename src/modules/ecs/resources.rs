pub mod shader;
pub mod texture;
pub mod camera;
pub mod tilemap;

pub use camera::RCamera2D;
pub use tilemap::RTileMap;
pub use shader::RShader;
pub use texture::RTexture;

use crate::Registry;

pub fn register_resources(registry: &mut Registry) {
    registry
        .register_resource::<RShader>()
        .register_resource::<RCamera2D>()
        .register_resource::<RTileMap>()
        .register_resource::<RTexture>();
}
