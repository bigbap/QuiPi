pub mod assets;
mod loaders;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    prelude::{
        qp_core::StringInterner, qp_ecs::{
            Component,
            EntityManager
        },
        QPError,
        VersionedIndex
    },
    QPResult
};

pub struct AssetManager {
    asset_store: EntityManager,
    asset_map: HashMap<u64, VersionedIndex>,

    string_interner: Rc<RefCell<StringInterner>>
}

impl AssetManager {
    pub fn init(
        string_interner: Rc<RefCell<StringInterner>>
    ) -> QPResult<Self> {
        let mut manager = Self {
            asset_store: EntityManager::new()?,
            asset_map: HashMap::new(),
            string_interner
        };

        manager.asset_store
            .register_component::<assets::RShader>()
            .register_component::<assets::RCamera2D>()
            .register_component::<assets::RTileMap>()
            .register_component::<assets::RTexture>()
            .register_component::<assets::RTextureAtlas>();

        Ok(manager)
    }

    pub fn load_asset<A: Component + std::fmt::Debug + PartialEq + 'static>(
        &mut self,
        name: String,
        asset: A
    ) -> QPResult<u64> {
        let id = self.string_interner.borrow_mut().intern(name);

        if self.asset_map.get(&id).is_some() {
            return Err(QPError::DuplicateAsset)
        }

        let index = self.asset_store.create();
        self.asset_store.add(&index, asset);

        self.asset_map.insert(id, index);

        Ok(id)
    }

    pub fn unload_asset<A: Component + std::fmt::Debug + PartialEq + 'static>(
        &mut self,
        id: u64
    ) {
        if let Some(index) = self.asset_map.get(&id) {
            self.asset_store.remove::<A>(index);

            self.asset_map.remove(&id);
        }
    }

    pub fn get<A: Component + std::fmt::Debug + PartialEq + 'static>(
        &self,
        id: u64
    ) -> Option<&A> {
        match self.asset_map.get(&id) {
            Some(index) => self.asset_store.get::<A>(index),
            None => None
        }
    }

    pub fn get_mut<A: Component + std::fmt::Debug + PartialEq + 'static>(
        &mut self,
        id: u64
    ) -> Option<&mut A> {
        match self.asset_map.get(&id) {
            Some(index) => self.asset_store.get_mut::<A>(index),
            None => None
        }
    }

    pub fn get_asset_id(&mut self, name: &str) -> Option<u64> {
        let id = self.string_interner.borrow_mut().intern(name.to_string());

        match self.asset_map.contains_key(&id) {
            true => Some(id),
            _ => None
        }
    }

    pub fn add_index(&mut self, id: u64, index: VersionedIndex) {
        self.asset_map.insert(id, index);
    }

    pub fn get_index(&self, id: u64) -> Option<VersionedIndex> {
        self.asset_map.get(&id).cloned()
    }

    pub fn flush(&mut self) {
        self.asset_store.flush();
    }

    pub fn register_asset<A: Component + std::fmt::Debug + PartialEq + 'static>(
        &mut self
    ) {
        self.asset_store.register_component::<A>();
    }
}
