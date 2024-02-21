pub mod camera2d;
pub mod sprite;
pub mod scene2d;
pub mod shader;
pub mod texture;

pub use camera2d::SchemaCamera2D;
pub use sprite::SchemaSprite;
pub use scene2d::SchemaScene2D;
pub use shader::SchemaShader;
pub use texture::SchemaTexture;

use crate::{
    Registry, VersionedIndex
};

pub trait ISchema: Clone {
    fn build_entity(&self, _registry: &mut Registry) -> Result<VersionedIndex, Box<dyn std::error::Error>> { unimplemented!() }
    fn from_entity(_entity: VersionedIndex, _registry: &Registry) -> Option<Self> { unimplemented!() }

    fn load_resource(&self, _registry: &mut Registry) -> Result<u64, Box<dyn std::error::Error>> { unimplemented!() }
    fn from_resource(_id: u64, _registry: &Registry) -> Option<Self> { unimplemented!() }
}
