pub mod assets;
mod loaders;

use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::{
    prelude::{
        qp_core::StringInterner,
        qp_ecs::{Component, EntityManager},
        Index, QPError,
    },
    QPResult,
};

pub struct AssetManager {
    asset_store: EntityManager,
    asset_map: HashMap<u64, Index>,

    strings: Weak<RefCell<StringInterner>>,
}

impl AssetManager {
    pub fn init(strings: Weak<RefCell<StringInterner>>) -> QPResult<Self> {
        let mut manager = Self {
            asset_store: EntityManager::new()?,
            asset_map: HashMap::new(),
            strings,
        };

        Ok(manager)
    }

    pub fn load_asset<A: Component + std::fmt::Debug + PartialEq + 'static>(
        &mut self,
        name: &str,
        asset: A,
    ) -> QPResult<u64> {
        let Some(interner) = self.string_interner() else {
            return Err(QPError::SharedReferenceDropped);
        };

        let id = interner.borrow_mut().intern(name.to_string());

        if self.asset_map.get(&id).is_none() {
            let index = self.asset_store.create(asset);

            self.asset_map.insert(id, index);
        } else {
            #[cfg(debug_assertions)]
            println!("tried to load an already loaded asset");
        }

        Ok(id)
    }

    pub fn unload_asset<A: Component + std::fmt::Debug + PartialEq + 'static>(&mut self, id: u64) {
        if let Some(index) = self.asset_map.get(&id) {
            self.asset_store.remove::<A>(index);

            self.asset_map.remove(&id);
        }
    }

    pub fn get<A: Component + std::fmt::Debug + PartialEq + 'static>(&self, id: u64) -> Option<&A> {
        match self.asset_map.get(&id) {
            Some(index) => self.asset_store.get::<A>(index),
            None => None,
        }
    }

    pub fn get_mut<A: Component + std::fmt::Debug + PartialEq + 'static>(
        &mut self,
        id: u64,
    ) -> Option<&mut A> {
        match self.asset_map.get(&id) {
            Some(index) => self.asset_store.get_mut::<A>(index),
            None => None,
        }
    }

    pub fn get_asset_id(&mut self, name: &str) -> Option<u64> {
        let Some(interner) = self.string_interner() else {
            return None;
        };

        let id = interner.borrow_mut().intern(name.to_string());

        match self.asset_map.contains_key(&id) {
            true => Some(id),
            _ => None,
        }
    }

    pub fn add_index(&mut self, id: u64, index: Index) {
        self.asset_map.insert(id, index);
    }

    pub fn get_index(&self, id: u64) -> Option<Index> {
        self.asset_map.get(&id).cloned()
    }

    pub fn flush(&mut self) {
        self.asset_store.flush();
    }

    fn string_interner(&mut self) -> Option<Rc<RefCell<StringInterner>>> {
        let Some(string_interner) = self.strings.upgrade() else {
            #[cfg(debug_assertions)]
            println!("[asset manager] weak reference to string_interner returned None");

            return None;
        };

        Some(string_interner)
    }
}
