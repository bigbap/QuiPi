mod shader;
mod texture;
mod camera;
mod tilemap;

pub mod resources {
    pub use super::*;

    pub use camera::RCamera2D;
    pub use tilemap::RTileMap;
    pub use shader::RShader;
    pub use texture::RTexture;

    use crate::prelude::Registry;

    pub fn register_resources(registry: &mut Registry) {
        registry
            .register_resource::<RShader>()
            .register_resource::<RCamera2D>()
            .register_resource::<RTileMap>()
            .register_resource::<RTexture>();
    }
}
