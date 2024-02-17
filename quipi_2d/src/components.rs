pub mod bounding_box;
pub mod camera;
pub mod matrices;
pub mod shapes;
pub mod transform;
pub mod velocity;

pub use bounding_box::CBoundingBox2D;
pub use camera::CCamera2D;
pub use matrices::CModelMatrix2D;
pub use matrices::CViewMatrix2D;
pub use shapes::CQuad;
pub use shapes::CCircle;
pub use transform::CTransform2D;
pub use velocity::CVelocity2D;

pub use quipi_core::components::*;
use quipi_core::Registry;

pub fn register_components(registry: &mut Registry) {
    quipi_core::components::register_components(registry);

    registry.entities
        .register_component::<CBoundingBox2D>()
        .register_component::<CCamera2D>()
        .register_component::<CCircle>()
        .register_component::<CModelMatrix2D>()
        .register_component::<CTransform2D>()
        .register_component::<CQuad>()
        .register_component::<CVelocity2D>()
        .register_component::<CViewMatrix2D>();
}

pub fn register_resources(registry: &mut Registry) {
    quipi_core::components::register_resources(registry);
}