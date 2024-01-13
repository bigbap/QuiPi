pub mod camera;
pub mod material;
pub mod shader;
pub mod texture;

pub use camera::Camera3D;
pub use camera::CameraProjection;
pub use material::Material;
pub use shader::Shader;
pub use texture::Texture;

use crate::Registry;

pub fn register_resource(registry: &mut Registry) {
    registry
        .register_resource::<Camera3D>()
        .register_resource::<Material>()
        .register_resource::<Shader>()
        .register_resource::<Texture>();
}
