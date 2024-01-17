pub use engine::components::CRGBA;
pub use engine::components::CTransform;
pub use engine::components::CMesh;

use engine::Registry;
pub fn register_components(registry: &mut Registry) {
    engine::components::register_components(registry);
}
