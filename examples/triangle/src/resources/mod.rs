pub use engine::resources::Shader;

pub fn register_resources(registry: &mut engine::Registry) {
    registry.register_resource::<Shader>(); 
}
