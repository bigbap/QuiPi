use std::{cell::RefCell, rc::Rc};

use super::{
    bundle::{Bundle, ComponentMap},
    indexed_array::{Allocator, Index, IndexedArray},
    prelude::Component,
};
use crate::QPResult;

type EntityMap<C> = IndexedArray<C>;

#[derive(Debug)]
pub struct EntityManager {
    allocator: Rc<RefCell<Allocator>>,
    components: ComponentMap,

    entities: Vec<Index>,
    to_delete: Vec<Index>,
}

impl EntityManager {
    pub fn new() -> Self {
        let entity_manager = Self {
            allocator: Rc::new(RefCell::new(Allocator::default())),
            components: ComponentMap::new(),

            entities: Vec::<Index>::new(),
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
        match self.components.get_mut::<EntityMap<C>>(C::id()) {
            None => (),
            Some(cmp_map) => {
                cmp_map.set(&entity, component);
            }
        }
    }

    pub fn remove<C: Component + std::fmt::Debug + PartialEq + 'static>(&mut self, entity: &Index) {
        match self.components.get_mut::<EntityMap<C>>(C::id()) {
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

        match self.components.get::<EntityMap<C>>(C::id()) {
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

        match self.components.get_mut::<EntityMap<C>>(C::id()) {
            None => None,
            Some(cmp_map) => match cmp_map.get_mut(entity) {
                None => None,
                Some(cmp) => Some(cmp),
            },
        }
    }

    pub fn get_all<C: Component + PartialEq + 'static>(&self) {}

    pub fn query<C: Component + PartialEq + 'static>(&self) -> Option<&EntityMap<C>> {
        self.components.get::<EntityMap<C>>(C::id())
    }
    pub fn query_mut<C: Component + PartialEq + 'static>(&mut self) -> Option<&mut EntityMap<C>> {
        self.components.get_mut::<EntityMap<C>>(C::id())
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

    pub fn reset(&mut self) -> QPResult<()> {
        for entity in self.entities.iter() {
            self.allocator.borrow_mut().deallocate(*entity);
        }

        self.entities.clear();

        Ok(())
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

    pub fn get_valid_entities(&mut self) -> Vec<Index> {
        let mut result = Vec::<Index>::new();

        for entity in self.entities.iter() {
            if self.allocator.borrow().validate(entity) {
                result.push(*entity);
            }
        }

        result
    }
}
