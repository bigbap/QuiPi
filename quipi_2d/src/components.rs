pub mod bounding_box;
pub mod camera;
pub mod matrices;
pub mod shapes;
pub mod sprite;
pub mod transform;
pub mod velocity;

pub use bounding_box::CBoundingBox2D;
pub use camera::CCamera2D;
pub use matrices::CModelMatrix2D;
pub use matrices::CViewMatrix2D;
pub use shapes::CRect;
pub use shapes::CCircle;
pub use sprite::CMesh2D;
pub use sprite::CSprite;
pub use transform::CTransform2D;
pub use velocity::CVelocity2D;

pub use quipi_core::components::{
    CScene,
    CRGBA,
    CName,
    CTag,
};
use quipi_core::Registry;

pub fn register_components(registry: &mut Registry) {
    // TODO: remove all 3d components from core and just call
    // register_components from core
    registry.entities
        .register_component::<CRGBA>()
        .register_component::<CScene>()
        .register_component::<CName>()
        .register_component::<CTag>();


    registry.entities
        .register_component::<CBoundingBox2D>()
        .register_component::<CCamera2D>()
        .register_component::<CCircle>()
        .register_component::<CModelMatrix2D>()
        .register_component::<CTransform2D>()
        .register_component::<CMesh2D>()
        .register_component::<CRect>()
        .register_component::<CSprite>()
        .register_component::<CVelocity2D>()
        .register_component::<CViewMatrix2D>();
}

pub fn register_resources(registry: &mut Registry) {
    quipi_core::components::register_resources(registry);
}