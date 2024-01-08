pub mod children;
pub mod color;
pub mod draw;
pub mod transform;

pub use children::ChildrenComponent;
pub use color::ColorComponent;
pub use draw::DrawComponent;
pub use transform::TransformComponent;

use engine::Registry;
pub fn register_components(registry: &mut Registry) {
    registry
        .register_component::<ChildrenComponent>()
        .register_component::<ColorComponent>()
        .register_component::<DrawComponent>()
        .register_component::<TransformComponent>()
        .register_component::<engine::gfx::components::MeshComponent>();
}
