pub mod shader;

pub use shader::SchemaShader;

use crate::{
    Registry, VersionedIndex
};

pub trait ISchema: Clone {
    fn build(&self, registry: &mut Registry) -> Result<VersionedIndex, Box<dyn std::error::Error>>;
    fn from_entity(entity: VersionedIndex, registry: &Registry) -> Option<Self>;
}
