pub use engine::components::Draw;
pub use engine::components::Children;
pub use engine::components::Color;
pub use engine::components::ModelTransform;
pub use engine::components::Mesh;

use engine::Registry;
pub fn register_components(registry: &mut Registry) {
    engine::components::register_components(registry);
}
