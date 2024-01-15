pub use engine::resources::Shader;

pub fn register_resources(registry: &mut engine::Registry) {
    engine::resources::register_resource(registry);
}
