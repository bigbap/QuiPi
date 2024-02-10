pub mod camera2d;
pub mod scene2d;
pub mod entity2d;
pub mod shader;

pub use camera2d::SchemaCamera2D;
pub use scene2d::SchemaScene2D;
pub use entity2d::SchemaEntity2D;
pub use shader::SchemaShader;

use std::fmt::Debug;
use crate::{
    ec_store::EMError, registry::RegistryError, Registry, VersionedIndex
};

pub trait ISchema: Clone {
    fn build(&self, registry: &mut Registry) -> Result<VersionedIndex, SchemaError>;
    fn from_entity(entity: VersionedIndex, registry: &Registry) -> Option<Self>;
}

#[derive(Debug, thiserror::Error)]
pub enum SchemaError {
    // SchemaRect errors
    #[error("[SchemaRect] shader not found")]
    ShaderNotFound,

    #[error("[SchemaRect] camera not found")]
    CameraNotFound,

    #[error("Registry error")]
    RegistryError(
        #[from]
        RegistryError
    ),

    #[error("EC Store error")]
    ECStoreError(
        #[from]
        EMError
    ),

    #[error("Other error")]
    OtherError(
        #[from]
        Box<dyn std::error::Error>
    ),
}
