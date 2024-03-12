use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{
    bundle::{Bundle, ComponentMap},
    indexed_array::{Allocator, Index, IndexedArray},
    prelude::Component,
};
use crate::{prelude::QPError, resources::*, QPResult};

#[derive(Debug, Resource)]
pub struct StorageManager {
    storage_units: HashMap<StorageId, Storage>,
}

impl StorageManager {
    pub fn new() -> Self {
        Self {
            storage_units: HashMap::new(),
        }
    }

    pub fn insert_storage_unit(&mut self, id: StorageId) -> QPResult<Option<Storage>> {
        if self.storage_units.contains_key(&id) {
            return Err(QPError::DuplicateStorage);
        }

        Ok(self.storage_units.insert(id, Storage::new()))
    }

    pub fn get<C: Component + PartialEq + 'static>(
        &self,
        storage: StorageId,
        index: &Index,
    ) -> Option<&C> {
        self.storage_units.get(&storage)?.get::<C>(&index)
    }

    pub fn get_mut<C: Component + PartialEq + 'static>(
        &mut self,
        storage: StorageId,
        index: &Index,
    ) -> Option<&mut C> {
        self.storage_units.get_mut(&storage)?.get_mut::<C>(&index)
    }

    pub fn query<C: Component + PartialEq + 'static>(
        &self,
        storage: StorageId,
    ) -> Option<&IndexedArray<C>> {
        self.storage_units.get(&storage)?.query::<C>()
    }

    pub fn query_mut<C: Component + PartialEq + 'static>(
        &mut self,
        storage: StorageId,
    ) -> Option<&mut IndexedArray<C>> {
        self.storage_units.get_mut(&storage)?.query_mut::<C>()
    }

    pub fn get_storage(&self, id: StorageId) -> Option<&Storage> {
        self.storage_units.get(&id)
    }

    pub fn get_storage_mut(&mut self, id: StorageId) -> Option<&mut Storage> {
        self.storage_units.get_mut(&id)
    }

    // pub fn query<C: Component + PartialEq + 'static>(&self, filter: C) -> Vec<Index> {
    //     let Some(cmp_map) = self.components.get::<EntityMap<C>>(C::id()) else {
    //         return vec![];
    //     };
    //     let all_entities = cmp_map.get_entities();

    //     let mut result = Vec::<Index>::new();

    //     for entity in all_entities {
    //         if let Some(cmp) = cmp_map.get(&entity) {
    //             if *cmp == filter {
    //                 result.push(entity);
    //             }
    //         };
    //     }

    //     result
    // }
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
pub enum StorageId {
    Entities,
    Cameras,
}

#[derive(Debug)]
pub struct Storage {
    allocator: Rc<RefCell<Allocator>>,
    components: ComponentMap,
    to_delete: Vec<Index>,
}

impl Storage {
    pub fn new() -> Self {
        let entity_manager = Self {
            allocator: Rc::new(RefCell::new(Allocator::default())),
            components: ComponentMap::new(),
            to_delete: Vec::<Index>::new(),
        };

        entity_manager
    }

    pub fn create<B: Bundle>(&mut self, bundle: B) -> Index {
        let entity = self.allocator.borrow_mut().allocate();

        bundle.add_components(
            &mut self.components,
            Rc::downgrade(&self.allocator),
            &entity,
        );

        entity
    }

    pub fn set_to_delete(&mut self, entity: Index) {
        self.to_delete.push(entity);
    }

    pub fn flush(&mut self) {
        for entity in self.to_delete.iter_mut() {
            self.allocator.borrow_mut().deallocate(*entity);
        }

        self.to_delete.clear();
    }

    pub fn add<C: Component + std::fmt::Debug + PartialEq + 'static>(
        &mut self,
        entity: &Index,
        component: C,
    ) {
        match self.components.get_mut::<IndexedArray<C>>(C::id()) {
            None => (),
            Some(cmp_map) => {
                cmp_map.set(&entity, component);
            }
        }
    }

    pub fn remove<C: Component + std::fmt::Debug + PartialEq + 'static>(&mut self, entity: &Index) {
        match self.components.get_mut::<IndexedArray<C>>(C::id()) {
            None => (),
            Some(cmp_map) => {
                cmp_map.unset(&entity);
            }
        }
    }

    pub fn get<C: Component + PartialEq + 'static>(&self, entity: &Index) -> Option<&C> {
        if !self.allocator.borrow().validate(entity) {
            return None;
        }

        match self.components.get::<IndexedArray<C>>(C::id()) {
            None => None,
            Some(cmp_map) => match cmp_map.get(entity) {
                None => None,
                Some(cmp) => Some(cmp),
            },
        }
    }

    pub fn get_mut<C: Component + PartialEq + 'static>(
        &mut self,
        entity: &Index,
    ) -> Option<&mut C> {
        if !self.allocator.borrow().validate(entity) {
            return None;
        }

        match self.components.get_mut::<IndexedArray<C>>(C::id()) {
            None => None,
            Some(cmp_map) => match cmp_map.get_mut(entity) {
                None => None,
                Some(cmp) => Some(cmp),
            },
        }
    }

    pub fn get_all<C: Component + PartialEq + 'static>(&self) {}

    pub fn query<C: Component + PartialEq + 'static>(&self) -> Option<&IndexedArray<C>> {
        self.components.get::<IndexedArray<C>>(C::id())
    }

    pub fn query_mut<C: Component + PartialEq + 'static>(
        &mut self,
    ) -> Option<&mut IndexedArray<C>> {
        self.components.get_mut::<IndexedArray<C>>(C::id())
    }

    pub fn clear(&mut self) -> QPResult<()> {
        Ok(self.allocator.borrow_mut().clear())
    }

    pub fn registered_components_len(&self) -> usize {
        self.components.len()
    }

    pub fn allocator_size(&self) -> usize {
        self.allocator.borrow().len()
    }

    pub fn count(&self) -> usize {
        self.allocator.borrow().valid_count()
    }
}
