use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{
    bundle::{Bundle, ComponentMap},
    indexed_array::{Allocator, Index, IndexedArray, Iter, IterMut},
    prelude::Component,
};
use crate::{
    prelude::{Group, QPError, Query, World},
    resources::*,
    QPResult,
};

#[derive(Debug, Resource, AsAny)]
pub struct StorageManager {
    storage_units: HashMap<StorageId, Storage>,
}

impl StorageManager {
    pub fn new() -> Self {
        Self {
            storage_units: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: StorageId) -> QPResult<()> {
        if self.storage_units.contains_key(&id) {
            return Err(QPError::DuplicateStorage);
        }

        self.storage_units.insert(id, Storage::new());

        Ok(())
    }

    pub fn get(&self, storage: StorageId) -> Option<&Storage> {
        self.storage_units.get(&storage)
    }

    pub fn get_mut(&mut self, storage: StorageId) -> Option<&mut Storage> {
        self.storage_units.get_mut(&storage)
    }

    pub(crate) fn flush(&mut self) {
        for storage in self.storage_units.values_mut() {
            storage.flush()
        }
    }

    // pub fn query<C: Component + 'static>(&self, storage: StorageId) -> Option<&IndexedArray<C>> {
    //     self.storage_units.get(&storage)?.query::<C>()
    // }

    // pub fn query_mut<C: Component + 'static>(
    //     &mut self,
    //     storage: StorageId,
    // ) -> Option<&mut IndexedArray<C>> {
    //     self.storage_units.get_mut(&storage)?.query_mut::<C>()
    // }

    // pub fn create<B: Bundle>(&mut self, storage: StorageId, bundle: B) -> Option<Index> {
    //     let storage = self.get_storage_mut(storage)?;

    //     Some(storage.spawn(bundle))
    // }

    // // pub fn query_v2<G: Group + 'static>(&'static mut self, storage: StorageId) -> Option<Query> {
    // //     let storage = self.get_storage(storage)?;
    // //     let query = Query {
    // //         nexer: G::into_iter(&storage).nexer(),
    // //     };

    // //     Some(query)
    // // }

    // fn get_storage(&self, id: StorageId) -> Option<&Storage> {
    //     match self.storage_units.get(&id) {
    //         Some(storage) => Some(storage),
    //         _ => None,
    //     }
    // }

    // fn get_storage_mut(&mut self, id: StorageId) -> Option<&mut Storage> {
    //     match self.storage_units.get_mut(&id) {
    //         Some(storage) => Some(storage),
    //         _ => None,
    //     }
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
        Self {
            allocator: Rc::new(RefCell::new(Allocator::default())),
            components: ComponentMap::new(),
            to_delete: Vec::<Index>::new(),
        }
    }

    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> Index {
        let entity = self.allocator.borrow_mut().allocate();

        bundle.add_components(
            &mut self.components,
            Rc::downgrade(&self.allocator),
            &entity,
        );

        entity
    }

    pub fn despwan(&mut self, entity: Index) {
        // println!("despawning {:?}", entity);
        self.to_delete.push(entity);
    }

    pub(crate) fn flush(&mut self) {
        for entity in self.to_delete.iter_mut() {
            self.allocator.borrow_mut().deallocate(*entity);
        }

        self.to_delete.clear();
    }

    pub(crate) fn add<C: Component + std::fmt::Debug + PartialEq + 'static>(
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

    pub(crate) fn remove<C: Component + std::fmt::Debug + PartialEq + 'static>(
        &mut self,
        entity: &Index,
    ) {
        match self.components.get_mut::<C>() {
            None => (),
            Some(cmp_map) => {
                cmp_map.unset(&entity);
            }
        }
    }

    // get a single component for a given entity
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

    // get a single mutable component for a given entity
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

    // get a single component list
    pub(crate) fn get_component_list<C: Component + PartialEq + 'static>(
        &self,
    ) -> Option<&IndexedArray<C>> {
        self.components.get::<C>()
    }

    // get a single mutable component list
    pub(crate) fn get_component_list_mut<C: Component + PartialEq + 'static>(
        &mut self,
    ) -> Option<&mut IndexedArray<C>> {
        self.components.get_mut::<C>()
    }

    // iterator over all components of a given type
    pub fn iter<C: Component + 'static>(&self) -> Option<Iter<'_, C>> {
        Some(self.get_component_list::<C>()?.iter())
    }

    // mutable iterator over all components of a given type
    pub fn iter_mut<C: Component + 'static>(&mut self) -> Option<IterMut<'_, C>> {
        Some(self.get_component_list_mut::<C>()?.iter_mut())
    }

    pub fn len(&self) -> usize {
        self.allocator.borrow().valid_count()
    }
}

impl World {
    pub fn get_entity_component<C: Component>(
        &self,
        storage: StorageId,
        entity: &Index,
    ) -> Option<&C> {
        self.storage_manager().get(storage)?.get::<C>(entity)
    }

    pub fn get_entity_component_mut<C: Component>(
        &mut self,
        storage: StorageId,
        entity: &Index,
    ) -> Option<&mut C> {
        self.storage_manager_mut()
            .get_mut(storage)?
            .get_mut::<C>(entity)
    }

    pub fn entity_iter<C: Component + 'static>(&self, storage: StorageId) -> Option<Iter<'_, C>> {
        self.storage_manager().get(storage)?.iter()
    }

    pub fn entity_iter_mut<C: Component + 'static>(
        &mut self,
        storage: StorageId,
    ) -> Option<IterMut<'_, C>> {
        self.storage_manager_mut().get_mut(storage)?.iter_mut()
    }
}
