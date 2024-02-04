pub mod camera;
pub mod scene;
pub mod rect;
pub mod shader;

pub use camera::SchemaCamera;
pub use scene::SchemaScene;
pub use rect::SchemaRect;
pub use shader::SchemaShader;

use std::fmt::Debug;
use crate::{
    Registry,
    VersionedIndex,
    registry::RegistryError
};

pub trait ISchema {
    fn build(&self, registry: &mut Registry) -> Result<VersionedIndex, SchemaError>;
}

pub trait IPrefab<T> {
    fn build_instance(
        &self,
        registry: &mut Registry,
        instance: &T
    ) -> Result<VersionedIndex, SchemaError>;
}

#[derive(Debug, thiserror::Error)]
pub enum SchemaError {
    // SchemaRect errors
    #[error("[SchemaRect] shader not found")]
    ShaderNotFound,

    #[error("Registry error")]
    RegistryError(
        #[from]
        RegistryError
    ),

    #[error("Other error")]
    OtherError(
        #[from]
        Box<dyn std::error::Error>
    ),
}
