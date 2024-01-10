pub use engine::resources::Shader;
pub use engine::resources::Camera3D;

pub fn register_resources(registry: &mut engine::Registry) {
    registry
        .register_resource::<Shader>()
        .register_resource::<Camera3D>(); 
}
