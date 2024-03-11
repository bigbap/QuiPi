use crate::{
    core::prelude::AsAny, platform::opengl::textures::Format, resources::Resource, QPResult,
};
use std::{any::TypeId, collections::HashMap};

pub mod prelude {
    pub use super::*;
}

#[derive(Debug)]
pub(crate) struct AssetStore<A: Asset + 'static> {
    store: HashMap<AssetId, A>,
}

impl<A: Asset> AssetStore<A> {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn load_asset(
        &mut self,
        mut loader: impl AssetLoader<A>,
        identifier: u64,
    ) -> QPResult<AssetId> {
        let id = self.asset_id(identifier);

        if self.store.get(&id).is_none() {
            let asset = loader.load()?;

            self.store.insert(id, asset);
        }

        Ok(id)
    }

    pub fn unload_asset(&mut self, id: AssetId) -> QPResult<()> {
        if let Some(asset) = self.store.get_mut(&id) {
            asset.unload()?;

            self.store.remove(&id);
        }

        Ok(())
    }

    pub fn get(&self, id: &AssetId) -> Option<&A> {
        self.store.get(&id)
    }

    pub fn get_mut(&mut self, id: &AssetId) -> Option<&mut A> {
        self.store.get_mut(&id)
    }

    pub fn asset_id(&self, identifier: u64) -> AssetId {
        AssetId {
            id: identifier,
            kind: std::any::TypeId::of::<A>(),
        }
    }
}

pub trait Asset {
    fn unload(&mut self) -> QPResult<()> {
        Ok(())
    }
}

pub trait AssetLoader<A: Asset> {
    fn load(&mut self) -> QPResult<A>;
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct AssetId {
    pub id: u64,
    pub kind: TypeId,
}

impl AssetId {
    pub fn validate<A: Asset + 'static>(&self) -> bool {
        self.kind == std::any::TypeId::of::<A>()
    }
}

impl<A: Asset> Resource for AssetStore<A> {}
impl<A: Asset> AsAny for AssetStore<A> {
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
