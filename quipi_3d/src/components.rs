pub mod bounding_box;
pub mod camera;
pub mod drawable;
pub mod gizmo;
pub mod lights;
pub mod material;
pub mod matrices;
pub mod target;
pub mod transform;
pub mod velocity;

pub use bounding_box::CBoundingBox;
pub use camera::CCamera;
pub use drawable::CDrawable;
pub use drawable::CMesh;
pub use gizmo::CGizmo;
pub use lights::CAttenuation;
pub use lights::CCutoff;
pub use material::CMaterial;
pub use matrices::CModelMatrix;
pub use matrices::CViewMatrix;
pub use target::CTarget;
pub use transform::CDirection;
pub use transform::CEulerAngles;
pub use transform::CTransform;
pub use velocity::CSpeed;
pub use velocity::CVelocity;

pub use quipi_core::components::*;
use quipi_core::Registry;

pub fn register_components(registry: &mut Registry) {
    quipi_core::components::register_components(registry);
    quipi_2d::components::register_components(registry);

    registry.entities
        .register_component::<CBoundingBox>()
        .register_component::<CCamera>()
        .register_component::<CDrawable>()
        .register_component::<CMesh>()
        .register_component::<CGizmo>()
        .register_component::<CAttenuation>()
        .register_component::<CCutoff>()
        .register_component::<CMaterial>()
        .register_component::<CModelMatrix>()
        .register_component::<CViewMatrix>()
        .register_component::<CTarget>()
        .register_component::<CDirection>()
        .register_component::<CEulerAngles>()
        .register_component::<CTransform>()
        .register_component::<CSpeed>()
        .register_component::<CVelocity>();
}

pub fn register_resources(registry: &mut Registry) {
    quipi_core::components::register_resources(registry);
}