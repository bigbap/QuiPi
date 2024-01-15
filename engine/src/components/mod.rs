pub mod children;
pub mod color;
pub mod draw;
pub mod lights;
pub mod mesh;
pub mod transform;

pub use children::Children;
pub use color::Color;
pub use draw::Draw;
pub use lights::LightDirectional;
pub use lights::LightPoint;
pub use lights::LightSpot;
pub use mesh::Mesh;
pub use transform::ModelTransform;

use crate::Registry;

pub fn register_components(registry: &mut Registry) {
    registry
        .register_component::<Children>()
        .register_component::<Color>()
        .register_component::<Draw>()
        .register_component::<LightDirectional>()
        .register_component::<LightPoint>()
        .register_component::<LightSpot>()
        .register_component::<Mesh>()
        .register_component::<ModelTransform>();
}
