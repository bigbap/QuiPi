pub mod circle;
pub mod quad;
pub mod sprite;
pub mod transform;
pub mod velocity;

pub use quad::CQuad;
pub use circle::CCircle;
pub use transform::CTransform2D;
pub use sprite::CSprite;
pub use velocity::CVelocity2D;

pub use quipi_core::components::*;
use quipi_core::Registry;

pub fn register_components(registry: &mut Registry) {
    quipi_core::components::register_components(registry);

    registry.entities
        .register_component::<CCircle>()
        .register_component::<CTransform2D>()
        .register_component::<CQuad>()
        .register_component::<CSprite>()
        .register_component::<CVelocity2D>();
}