pub mod camera2d;
pub mod scene;
pub mod scene2d;
pub mod shader;
pub mod sprite;
pub mod texture;

pub mod prelude {
    use crate::{prelude::VersionedIndex, registry::GlobalRegistry, QPResult};

    use super::*;

    pub use camera2d::SchemaCamera2D;
    pub use scene2d::SchemaScene2D;
    pub use shader::SchemaShader;
    pub use sprite::SchemaSprite;
    pub use texture::SchemaTexture;

    pub use scene::*;

    pub trait Schema: Clone {
        fn build_entity(&self, _registry: &mut GlobalRegistry) -> QPResult<VersionedIndex> {
            unimplemented!()
        }

        fn from_entity(_entity: VersionedIndex, _registry: &GlobalRegistry) -> Option<Self> {
            unimplemented!()
        }

        fn load_resource(&self, _registry: &mut GlobalRegistry) -> QPResult<u64> {
            unimplemented!()
        }

        fn from_resource(_id: u64, _registry: &GlobalRegistry) -> Option<Self> {
            unimplemented!()
        }
    }
}
