pub mod children;
pub mod color;
pub mod lights;
pub mod mesh;
pub mod transform;

pub use children::ChildrenComponent;
pub use color::ColorComponent;
pub use lights::LightDirectionalComponent;
pub use lights::LightPointComponent;
pub use lights::LightSpotComponent;
pub use mesh::MeshComponent;
pub use transform::TransformComponent;

use crate::Registry;

pub fn register_components(registry: &mut Registry) {
    registry
        .register_component::<ChildrenComponent>()
        .register_component::<ColorComponent>()
        .register_component::<LightDirectionalComponent>()
        .register_component::<LightPointComponent>()
        .register_component::<LightSpotComponent>()
        .register_component::<MeshComponent>()
        .register_component::<TransformComponent>();
}
