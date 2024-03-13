pub mod assets;
mod loaders;

use std::collections::HashMap;

use crate::{resources::Resource, QPResult};

#[derive(Debug)]
pub struct AssetStore<A: Asset + 'static> {
    store: HashMap<AssetId, A>,
}

impl<A: Asset> AssetStore<A> {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn load_asset(&mut self, mut asset: A) -> QPResult<AssetId> {
        let id = asset.identifier();

        if self.store.get(&id).is_none() {
            asset.load(|state| {
                self.store.insert(id, asset);
            });
        } else {
            #[cfg(debug_assertions)]
            println!("tried to load an already loaded asset");
        }

        Ok(id)
    }

    pub fn unload_asset(&mut self, id: AssetId) {
        if let Some(asset) = self.store.get_mut(&id) {
            asset.unload(|state| {
                self.store.remove(&id);
            })
        }
    }

    pub fn get(&self, id: AssetId) -> Option<&A> {
        self.store.get(&id)
    }

    pub fn get_mut(&mut self, id: AssetId) -> Option<&mut A> {
        self.store.get_mut(&id)
    }

    // pub fn get_asset_id(&mut self, name: &str) -> Option<u64> {
    //     let Some(interner) = self.string_interner() else {
    //         return None;
    //     };

    //     let id = interner.borrow_mut().intern(name.to_string());

    //     match self.asset_map.contains_key(&id) {
    //         true => Some(id),
    //         _ => None,
    //     }
    // }

    // pub fn add_index(&mut self, id: u64, index: Index) {
    //     self.asset_map.insert(id, index);
    // }

    // pub fn get_index(&self, id: u64) -> Option<Index> {
    //     self.asset_map.get(&id).cloned()
    // }

    // pub fn flush(&mut self) {
    //     self.asset_store.flush();
    // }

    // fn string_interner(&mut self) -> Option<Rc<RefCell<StringInterner>>> {
    //     let Some(string_interner) = self.strings.upgrade() else {
    //         #[cfg(debug_assertions)]
    //         println!("[asset manager] weak reference to string_interner returned None");

    //         return None;
    //     };

    //     Some(string_interner)
    // }
}

pub trait Asset {
    fn identifier(&self) -> AssetId;

    fn load(&mut self, handler: impl AssetHandler) {
        handler(AssetState::Loaded(self.identifier()))
    }

    fn unload(&mut self, handler: impl AssetHandler) {
        handler(AssetState::Unloaded)
    }
}

pub enum AssetState {
    Loaded(AssetId),
    Unloaded,
}

pub trait AssetHandler: FnOnce(AssetState) + 'static {}
impl<F> AssetHandler for F where F: FnOnce(AssetState) + 'static {}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct AssetId(pub &'static str);

impl<A: Asset> Resource for AssetStore<A> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
