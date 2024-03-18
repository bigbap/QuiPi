use std::marker::PhantomData;

use crate::QPResult;
pub use macros::Asset;

pub trait Asset {}

pub trait AssetLoader {
    type AssetType: Asset;

    fn load(&mut self) -> QPResult<Self::AssetType>;
}

#[derive(Debug, Hash, Eq, Clone, Copy, PartialEq)]
pub enum AssetId {
    Id(u64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AssetHandle<A: Asset> {
    pub id: AssetId,

    pub marker: PhantomData<A>,
}

impl<A: Asset> AssetHandle<A> {
    pub fn new(id: AssetId) -> Self {
        Self {
            id,
            marker: PhantomData,
        }
    }
}
