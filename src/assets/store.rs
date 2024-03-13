use crate::resources::{AsAny, Resource};

use super::{Asset, AssetHandle, AssetId};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Assets<A: Asset + 'static> {
    store: HashMap<AssetId, A>,
}

impl<A: Asset + 'static> Default for Assets<A> {
    fn default() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl<A: Asset + 'static> Assets<A> {
    pub fn get(&self, id: &AssetId) -> Option<&A> {
        self.store.get(&id)
    }

    pub fn get_mut(&mut self, id: &AssetId) -> Option<&mut A> {
        self.store.get_mut(&id)
    }

    pub fn add(&mut self, id: u64, asset: A) -> AssetHandle<A> {
        let id = AssetId::Id(id);
        self.store.insert(id, asset);

        AssetHandle::new(id)
    }
}

impl<A: Asset + 'static> Resource for Assets<A> {}
impl<A: Asset + 'static> AsAny for Assets<A> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
