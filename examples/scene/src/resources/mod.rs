pub use engine::resources::Shader;
pub use engine::resources::Camera3D;
pub use engine::resources::Texture;

pub fn register_resources(registry: &mut engine::Registry) {
    engine::resources::register_resource(registry);
}
