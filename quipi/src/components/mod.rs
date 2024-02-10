pub mod bounding_box;
pub mod camera;
pub mod children;
pub mod color;
pub mod distance;
pub mod drawable;
pub mod gizmo;
pub mod identifiers;
pub mod lights;
pub mod material;
pub mod matrices;
pub mod model;
pub mod scene;
pub mod shapes;
pub mod speed;
pub mod states;
pub mod target;
pub mod transform;
pub mod resources;
pub mod unique_id;

pub use resources::*;

pub use bounding_box::CBoundingBox;
pub use camera::CCamera;
pub use children::CChildren;
pub use color::CRGBA;
pub use drawable::CDrawable;
pub use drawable::CMesh;
pub use identifiers::CName;
pub use identifiers::CTag;
pub use lights::CAttenuation;
pub use lights::CCutoff;
pub use material::CMaterial;
pub use gizmo::CGizmo3D;
pub use distance::CDistance;
pub use matrices::CModelMatrix;
pub use matrices::CViewMatrix;
pub use model::CModelNode;
pub use scene::CScene;
pub use shapes::CRect;
pub use shapes::CCircle;
pub use speed::CSpeed;
pub use speed::CVelocity;
pub use states::CMouseBtnState;
pub use transform::CDirection;
pub use transform::CEulerAngles;
pub use transform::CTransform;
pub use target::CTarget;
pub use unique_id::CUniqueId;

use crate::Registry;

pub fn register_components(registry: &mut Registry) {
    registry.entities
        .register_component::<CCamera>()
        .register_component::<CAttenuation>()
        .register_component::<CBoundingBox>()
        .register_component::<CRGBA>()
        .register_component::<CChildren>()
        .register_component::<CCutoff>()
        .register_component::<CDirection>()
        .register_component::<CDistance>()
        .register_component::<CDrawable>()
        .register_component::<CEulerAngles>()
        .register_component::<CGizmo3D>()
        .register_component::<CMaterial>()
        .register_component::<CMesh>()
        .register_component::<CModelMatrix>()
        .register_component::<CModelNode>()
        .register_component::<CMouseBtnState>()
        .register_component::<CName>()
        .register_component::<CRect>()
        .register_component::<CCircle>()
        .register_component::<CScene>()
        .register_component::<CSpeed>()
        .register_component::<CTag>()
        .register_component::<CTarget>()
        .register_component::<CTransform>()
        .register_component::<CVelocity>()
        .register_component::<CViewMatrix>()
        .register_component::<CUniqueId>();
}
