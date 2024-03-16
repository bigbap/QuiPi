use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{
    bundle::{Bundle, ComponentMap},
    indexed_array::{Allocator, Index, IndexedArray, Iter, IterMut},
    prelude::Component,
};
use crate::{prelude::QPError, resources::*, QPResult};

#[derive(Debug, Resource, AsAny)]
pub struct StorageManager {
    storage_units: HashMap<StorageId, &'static mut Storage>,
}

impl StorageManager {
    pub fn new() -> Self {
        Self {
            storage_units: HashMap::new(),
        }
    }

    pub fn insert_storage_unit(&mut self, id: StorageId) -> QPResult<()> {
        if self.storage_units.contains_key(&id) {
            return Err(QPError::DuplicateStorage);
        }

        self.storage_units
            .insert(id, Box::leak(Box::new(Storage::new())));

        Ok(())
    }

    pub fn get<C: Component>(&self, storage: StorageId, index: &Index) -> Option<&C> {
        self.storage_units.get(&storage)?.get::<C>(&index)
    }

    pub fn get_mut<C: Component + 'static>(
        &mut self,
        storage: StorageId,
        index: &Index,
    ) -> Option<&mut C> {
        self.storage_units.get_mut(&storage)?.get_mut::<C>(&index)
    }

    pub fn query<C: Component + 'static>(&self, storage: StorageId) -> Option<&IndexedArray<C>> {
        self.storage_units.get(&storage)?.query::<C>()
    }

    pub fn query_mut<C: Component + 'static>(
        &mut self,
        storage: StorageId,
    ) -> Option<&mut IndexedArray<C>> {
        self.storage_units.get_mut(&storage)?.query_mut::<C>()
    }

    pub fn create<B: Bundle>(&mut self, storage: StorageId, bundle: B) -> Option<Index> {
        let storage = self.get_storage_mut(storage)?;

        Some(storage.create(bundle))
    }

    // pub fn query_v2<G: Group + 'static>(&'static mut self, storage: StorageId) -> Option<Query> {
    //     let storage = self.get_storage(storage)?;
    //     let query = Query {
    //         nexer: G::into_iter(&storage).nexer(),
    //     };

    //     Some(query)
    // }

    fn get_storage(&self, id: StorageId) -> Option<&Storage> {
        match self.storage_units.get(&id) {
            Some(storage) => Some(storage),
            _ => None,
        }
    }

    fn get_storage_mut(&mut self, id: StorageId) -> Option<&mut Storage> {
        match self.storage_units.get_mut(&id) {
            Some(storage) => Some(storage),
            _ => None,
        }
    }
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
        Self {
            allocator: Rc::new(RefCell::new(Allocator::default())),
            components: ComponentMap::new(),
            to_delete: Vec::<Index>::new(),
        }
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
        match self.components.get_mut::<C>() {
            None => (),
            Some(cmp_map) => {
                cmp_map.set(&entity, component);
            }
        }
    }

    pub fn remove<C: Component + std::fmt::Debug + PartialEq + 'static>(&mut self, entity: &Index) {
        match self.components.get_mut::<C>() {
            None => (),
            Some(cmp_map) => {
                cmp_map.unset(&entity);
            }
        }
    }

    pub fn get<C: Component + PartialEq>(&self, entity: &Index) -> Option<&C> {
        if !self.allocator.borrow().validate(entity) {
            return None;
        }

        match self.components.get::<C>() {
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

        match self.components.get_mut::<C>() {
            None => None,
            Some(cmp_map) => match cmp_map.get_mut(entity) {
                None => None,
                Some(cmp) => Some(cmp),
            },
        }
    }

    pub fn query<C: Component + PartialEq + 'static>(&self) -> Option<&IndexedArray<C>> {
        self.components.get::<C>()
    }

    pub fn query_mut<C: Component + PartialEq + 'static>(
        &mut self,
    ) -> Option<&mut IndexedArray<C>> {
        self.components.get_mut::<C>()
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

    pub fn iter<C: Component + 'static>(&'static self) -> Option<Iter<'_, C>> {
        Some(self.query::<C>()?.iter())
    }

    pub fn iter_mut<C: Component + 'static>(&mut self) -> Option<IterMut<'_, C>> {
        Some(self.query_mut::<C>()?.iter_mut())
    }
}
