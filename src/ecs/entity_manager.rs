use std::{cell::RefCell, rc::Rc};

use super::{
    indexed_array::{Allocator, Index, IndexedArray},
    prelude::Component,
};
use crate::{prelude::qp_core::AnyMap, QPResult};

type EntityMap<C> = IndexedArray<C>;

#[derive(Debug)]
pub struct EntityManager {
    entity_allocator: Rc<RefCell<Allocator>>,
    component_maps: AnyMap,

    entities: Vec<Index>,
    to_delete: Vec<Index>,
}

impl EntityManager {
    pub fn new() -> QPResult<Self> {
        let entity_manager = Self {
            entity_allocator: Rc::new(RefCell::new(Allocator::default())),
            component_maps: AnyMap::new(),
            entities: Vec::<Index>::new(),
            to_delete: Vec::<Index>::new(),
        };

        Ok(entity_manager)
    }

    pub fn register_component<C: Component + PartialEq + 'static>(&mut self) -> &mut Self {
        self.component_maps
            .insert::<EntityMap<C>>(EntityMap::<C>::new(Rc::clone(&self.entity_allocator)));

        self
    }

    pub fn create(&mut self) -> Index {
        let entity = self.entity_allocator.borrow_mut().allocate();

        self.entities.push(entity);

        entity
    }

    pub fn set_to_delete(&mut self, entity: Index) {
        self.to_delete.push(entity);
    }

    pub fn flush(&mut self) {
        for entity in self.to_delete.iter_mut() {
            self.entity_allocator.borrow_mut().deallocate(*entity);
        }

        self.to_delete.clear();
    }

    pub fn add<C: Component + std::fmt::Debug + PartialEq + 'static>(
        &mut self,
        entity: &Index,
        component: C,
    ) {
        match self.component_maps.get_mut::<EntityMap<C>>() {
            None => {
                #[cfg(debug_assertions)]
                println!(
                    "component {:?} has not been registered",
                    std::any::type_name::<C>()
                );
            }
            Some(cmp_map) => {
                cmp_map.set(&entity, component);
            }
        }
    }

    pub fn remove<C: Component + std::fmt::Debug + PartialEq + 'static>(&mut self, entity: &Index) {
        match self.component_maps.get_mut::<EntityMap<C>>() {
            None => {
                #[cfg(debug_assertions)]
                println!(
                    "component {:?} has not been registered",
                    std::any::type_name::<C>()
                );
            }
            Some(cmp_map) => {
                cmp_map.unset(&entity);
            }
        }
    }

    pub fn get<C: Component + PartialEq + 'static>(&self, entity: &Index) -> Option<&C> {
        if !self.entity_allocator.borrow().validate(entity) {
            return None;
        }

        match self.component_maps.get::<EntityMap<C>>() {
            None => {
                #[cfg(debug_assertions)]
                println!(
                    "component {:?} has not been registered",
                    std::any::type_name::<C>()
                );

                return None;
            }
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
        if !self.entity_allocator.borrow().validate(entity) {
            return None;
        }

        match self.component_maps.get_mut::<EntityMap<C>>() {
            None => {
                #[cfg(debug_assertions)]
                println!(
                    "component {:?} has not been registered",
                    std::any::type_name::<C>()
                );

                return None;
            }
            Some(cmp_map) => match cmp_map.get_mut(entity) {
                None => None,
                Some(cmp) => Some(cmp),
            },
        }
    }

    pub fn get_all<C: Component + PartialEq + 'static>(&self) {}

    pub fn query_all<C: Component + PartialEq + 'static>(&self) -> Vec<Index> {
        let Some(cmp_map) = self.component_maps.get::<EntityMap<C>>() else {
            return vec![];
        };

        cmp_map.get_entities()
    }

    pub fn query<C: Component + PartialEq + 'static>(&self, filter: C) -> Vec<Index> {
        let Some(cmp_map) = self.component_maps.get::<EntityMap<C>>() else {
            return vec![];
        };
        let all_entities = cmp_map.get_entities();

        let mut result = Vec::<Index>::new();

        for entity in all_entities {
            if let Some(cmp) = cmp_map.get(&entity) {
                if *cmp == filter {
                    result.push(entity);
                }
            };
        }

        result
    }

    pub fn reset(&mut self) -> QPResult<()> {
        for entity in self.entities.iter() {
            self.entity_allocator.borrow_mut().deallocate(*entity);
        }

        self.entities.clear();

        Ok(())
    }

    pub fn registered_components_len(&self) -> usize {
        self.component_maps.len()
    }

    pub fn allocator_size(&self) -> usize {
        self.entity_allocator.borrow().length()
    }

    pub fn count(&self) -> usize {
        self.entity_allocator.borrow().valid_count()
    }

    pub fn get_valid_entities(&mut self) -> Vec<Index> {
        let mut result = Vec::<Index>::new();

        for entity in self.entities.iter() {
            if self.entity_allocator.borrow().validate(entity) {
                result.push(*entity);
            }
        }

        result
    }
}

pub struct EntityBuilder<'a> {
    entity_manager: &'a mut EntityManager,
    entity: Index,
}

impl<'a> EntityBuilder<'a> {
    pub fn create(entity_manager: &'a mut EntityManager) -> Self {
        let entity = entity_manager.entity_allocator.borrow_mut().allocate();

        entity_manager.entities.push(entity);

        Self {
            entity_manager,
            entity,
        }
    }

    pub fn with<C: Component + std::fmt::Debug + PartialEq + 'static>(self, component: C) -> Self {
        self.entity_manager.add(&self.entity, component);

        self
    }

    pub fn build(self) -> Index {
        self.entity
    }
}
