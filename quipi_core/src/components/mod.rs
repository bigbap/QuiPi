pub mod children;
pub mod color;
pub mod distance;
pub mod drawable;
pub mod identifiers;
pub mod mesh;
// pub mod model;
pub mod scene;
pub mod states;
pub mod resources;
pub mod texture;
pub mod unique_id;

pub use resources::*;

pub use children::CChildren;
pub use color::CRGBA;
pub use drawable::CDrawable;
pub use identifiers::CName;
pub use identifiers::CTag;
pub use distance::CDistance;
pub use mesh::CMeshData;
// pub use model::CModelNode;
pub use scene::CScene;
pub use states::CMouseBtnState;
pub use texture::CTexture;
pub use unique_id::CUniqueId;

use crate::Registry;

pub fn register_components(registry: &mut Registry) {
    registry.entities
        .register_component::<CRGBA>()
        .register_component::<CChildren>()
        .register_component::<CDistance>()
        .register_component::<CDrawable>()
        // .register_component::<CModelNode>()
        .register_component::<CMeshData>()
        .register_component::<CMouseBtnState>()
        .register_component::<CName>()
        .register_component::<CScene>()
        .register_component::<CTag>()
        .register_component::<CTexture>()
        .register_component::<CUniqueId>();
}
