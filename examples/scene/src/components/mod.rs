pub mod draw;

pub use draw::DrawComponent;
pub use engine::components::CameraComponent;
pub use engine::components::ChildrenComponent;
pub use engine::components::ColorComponent;
pub use engine::components::TransformComponent;
pub use engine::components::MeshComponent;

use engine::Registry;
pub fn register_components(registry: &mut Registry) {
    registry
        .register_component::<CameraComponent>()
        .register_component::<ChildrenComponent>()
        .register_component::<ColorComponent>()
        .register_component::<DrawComponent>()
        .register_component::<TransformComponent>()
        .register_component::<MeshComponent>();
}
