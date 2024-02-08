use std::any::TypeId;
use crate::{
    core::collections::AnyMap,
    Component,
    VersionedIndex,
    ec_store::{
        IndexedArray,
        VersionedIndexAllocator
    }
};

type EntityMap<C> = IndexedArray<C>;

#[derive(Debug, thiserror::Error)]
pub enum EMError {
    #[error("there was a problem creating a new component registry")]
    ProblemCreatingNewComponentRegistry
}

#[derive(Debug)]
pub struct EntityManager {
    entity_allocator: VersionedIndexAllocator,
    component_maps: AnyMap,

    entities: Vec<VersionedIndex>,
    to_delete: Vec<VersionedIndex>,
    component_types: Vec<TypeId>
}

impl EntityManager {
    pub fn new() -> Result<Self, EMError> {
        let entity_manager = Self {
            entity_allocator: VersionedIndexAllocator::default(),
            component_maps: AnyMap::new(),
            entities: Vec::<VersionedIndex>::new(),
            to_delete: Vec::<VersionedIndex>::new(),
            component_types: Vec::<TypeId>::new()
        };

        Ok(entity_manager)
    }

    pub fn register_component<C: Component + PartialEq + 'static>(&mut self) -> &mut Self {
        self.component_maps.insert::<EntityMap<C>>(EntityMap::<C>::default());
        self.component_types.push(std::any::TypeId::of::<C>());

        self
    }

    pub fn create(&mut self) -> Result<VersionedIndex, EMError> {
        let entity = self.entity_allocator.allocate();

        self.entities.push(entity);

        Ok(entity)
    }

    pub fn set_to_delete(&mut self, entity: VersionedIndex) {
        self.to_delete.push(entity);
    }

    pub fn flush(&mut self) {
        for entity in self.to_delete.iter_mut() {
            self.entity_allocator.deallocate(*entity);
        }

        self.to_delete.clear();
    }

    pub fn add<C: Component + std::fmt::Debug + PartialEq + 'static>(
        &mut self,
        entity: &VersionedIndex,
        component: C
    ) {
        match self.component_maps.get_mut::<EntityMap<C>>() {
            None => {
                #[cfg(debug_assertions)]
                println!("component {:?} has not been registered", std::any::type_name::<C>());
            },
            Some(cmp_map) => { cmp_map.set(&entity, component); }
        }
    }

    pub fn get<C: Component + PartialEq + 'static>(
        &self,
        entity: &VersionedIndex
    ) -> Option<&C> {
        if !self.entity_allocator.validate(entity) {
            return None;
        }

        match self.component_maps.get::<EntityMap<C>>() {
            None => {
                #[cfg(debug_assertions)]
                println!("component {:?} has not been registered", std::any::type_name::<C>());

                return None;
            },
            Some(cmp_map) => match cmp_map.get(entity) {
                None => None,
                Some(cmp) => Some(cmp)
            }
        }
    }

    pub fn get_mut<C: Component + PartialEq + 'static>(
        &mut self,
        entity: &VersionedIndex
    ) -> Option<&mut C> {
        if !self.entity_allocator.validate(entity) {
            return None;
        }

        match self.component_maps.get_mut::<EntityMap<C>>() {
            None => {
                #[cfg(debug_assertions)]
                println!("component {:?} has not been registered", std::any::type_name::<C>());

                return None;
            },
            Some(cmp_map) => match cmp_map.get_mut(entity) {
                None => None,
                Some(cmp) => Some(cmp)
            }
        }
    }

    pub fn query_all<C: Component + PartialEq + 'static>(
        &self,
    ) -> Vec<VersionedIndex> {
        let Some(cmp_map) = self.component_maps.get::<EntityMap<C>>() else { return vec![] };
        
        cmp_map.get_entities(&self.entity_allocator)
    }

    pub fn query<C: Component + PartialEq + 'static>(&self, filter: C) -> Vec<VersionedIndex> {
        let Some(cmp_map) = self.component_maps.get::<EntityMap<C>>() else { return vec![] };
        let all_entities = cmp_map.get_entities(&self.entity_allocator);

        let mut result = Vec::<VersionedIndex>::new();

        for entity in all_entities {
            if let Some(cmp) = cmp_map.get(&entity) {
                if *cmp == filter {
                    result.push(entity);
                }
            };
        }

        result
    }

    pub fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for entity in self.entities.iter() {
            self.entity_allocator.deallocate(*entity);
        }

        self.entities.clear();

        Ok(())
    }

    pub fn registered_components_len(&self) -> usize {
        self.component_maps.len()
    }

    pub fn allocator_size(&self) -> usize {
        self.entity_allocator.length()
    }

    pub fn count(&self) -> usize {
        self.entity_allocator.valid_count()
    }

    pub fn get_valid_entities(&mut self) -> Vec<VersionedIndex> {
        let mut result = Vec::<VersionedIndex>::new();

        for entity in self.entities.iter() {
            if self.entity_allocator.validate(entity) {
                result.push(*entity);
            }
        }

        result
    }
}