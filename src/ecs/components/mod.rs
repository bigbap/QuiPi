mod children;
mod distance;
mod euler_angles;
mod identifiers;
mod gizmo;
mod mesh;
mod mvp;
mod scene;
mod states;
mod circle;
mod quad;
mod sprite;
mod target;
mod transform;
mod velocity;

pub mod components {
    use super::*;

    pub use quad::CQuad;
    pub use circle::CCircle;
    pub use distance::CDistance;
    pub use euler_angles::CEulerAngles;
    pub use gizmo::CGizmo;
    pub use transform::CTransform;
    pub use transform::CTransform2D;
    pub use sprite::CSprite;
    pub use velocity::CVelocity;
    pub use velocity::CVelocity2D;
    pub use children::CChildren;
    pub use identifiers::CTag;
    pub use mvp::CModelMatrix;
    pub use mvp::CProjectionMatrix;
    pub use mvp::CViewMatrix;
    pub use mvp::CMVPMatrix;
    pub use mesh::CMeshData;
    pub use scene::CScene;
    pub use states::CMouseBtnState;
    pub use target::CTarget;

    use crate::prelude::GlobalRegistry;

    pub fn register_components(registry: &mut GlobalRegistry) {
        registry.entity_manager
            .register_component::<CChildren>()
            .register_component::<CDistance>()
            .register_component::<CEulerAngles>()
            .register_component::<CGizmo>()
            .register_component::<CMeshData>()
            .register_component::<CModelMatrix>()
            .register_component::<CProjectionMatrix>()
            .register_component::<CViewMatrix>()
            .register_component::<CMVPMatrix>()
            .register_component::<CMouseBtnState>()
            .register_component::<CScene>()
            .register_component::<CTag>()
            .register_component::<CCircle>()
            .register_component::<CTransform>()
            .register_component::<CTransform2D>()
            .register_component::<CQuad>()
            .register_component::<CSprite>()
            .register_component::<CTarget>()
            .register_component::<CVelocity>()
            .register_component::<CVelocity2D>()
            .register_component::<()>(); // empty component
    }
}
