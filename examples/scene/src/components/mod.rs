pub mod draw;

pub use draw::DrawComponent;
pub use engine::components::ChildrenComponent;
pub use engine::components::ColorComponent;
pub use engine::components::TransformComponent;
pub use engine::components::MeshComponent;

use engine::Registry;
pub fn register_components(registry: &mut Registry) {
    engine::components::register_components(registry);
    registry.register_component::<DrawComponent>();
}
