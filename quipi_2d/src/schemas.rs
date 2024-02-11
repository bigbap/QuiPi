pub mod camera2d;
pub mod entity2d;
pub mod scene2d;

pub use camera2d::SchemaCamera2D;
pub use entity2d::SchemaEntity2D;
pub use scene2d::SchemaScene2D;

pub use quipi_core::schemas::{
    ISchema,
    SchemaShader
};