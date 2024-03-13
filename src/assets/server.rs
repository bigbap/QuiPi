use crate::{
    core::prelude::AsAny, platform::opengl::textures::Format, resources::Resource, QPResult,
};

use super::{AssetHandle, AssetLoader};

pub mod prelude {
    pub use super::*;
}

/// The asset server is a resource that loads and serves assets
/// (Texture, Shader, etc)
#[derive(Debug)]
pub struct AssetServer {}

impl AssetServer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load<L: AssetLoader>(&mut self, mut loader: L) -> QPResult<L::AssetType> {
        loader.load()
    }
}

// #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
// pub struct AssetId(pub u64, TypeId);
// impl AssetId {
//     pub fn validate<A: Asset + 'static>(&self) -> bool {
//         self.1 == std::any::TypeId::of::<A>()
//     }
// }

impl Resource for AssetServer {}
impl AsAny for AssetServer {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Source {
    Path(&'static str),
    Strings((&'static str, &'static str)),
    Buffer(BufferMetadata),
}

#[derive(Debug, PartialEq, Clone)]
pub struct BufferMetadata {
    pub format: Format,
    pub width: i32,
    pub height: i32,
    pub buffer: Vec<u8>,
}
