pub mod children;
pub mod color;
pub mod drawable;
pub mod identifiers;
pub mod mesh;
pub mod scene;
pub mod states;

pub use children::CChildren;
pub use color::CRGBA;
pub use drawable::CDrawable;
pub use identifiers::CTag;
pub use mesh::CMeshData;
pub use scene::CScene;
pub use states::CMouseBtnState;

use crate::Registry;

pub fn register_components(registry: &mut Registry) {
    registry.entities
        .register_component::<CRGBA>()
        .register_component::<CChildren>()
        .register_component::<CDrawable>()
        .register_component::<CMeshData>()
        .register_component::<CMouseBtnState>()
        .register_component::<CScene>()
        .register_component::<CTag>()
        .register_component::<()>(); // empty component
}
