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

pub use quipi_core::schema::ISchema;