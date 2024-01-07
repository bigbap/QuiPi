pub mod children;
pub mod color;
pub mod draw;
pub mod transform;

pub use children::ChildrenComponent;
pub use color::ColorComponent;
pub use draw::DrawComponent;
pub use transform::TransformComponent;

use engine::Registry;
pub fn register_components(registry: &Registry) {
    let mut reg_cmp = registry.components.borrow_mut();

    reg_cmp.register_component::<ChildrenComponent>(); 
    reg_cmp.register_component::<ColorComponent>(); 
    reg_cmp.register_component::<DrawComponent>(); 
    reg_cmp.register_component::<TransformComponent>();

    reg_cmp.register_component::<engine::gfx::components::MeshComponent>()
}
