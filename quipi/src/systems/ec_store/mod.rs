pub mod indexed_array;

pub use indexed_array::VersionedIndex;
pub use indexed_array::VersionedIndexAllocator;
pub use indexed_array::IndexedArray;
pub use component_derive::Component;

pub trait Component {
    fn my_type(&self) -> String;
}

use anymap2::AnyMap;

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
}

impl EntityManager {
    pub fn new() -> Result<Self, EMError> {
        let entity_manager = Self {
            entity_allocator: VersionedIndexAllocator::default(),
            component_maps: AnyMap::new(),
            entities: Vec::<VersionedIndex>::new(),
        };

        Ok(entity_manager)
    }

    pub fn register_component<C: Component + PartialEq + 'static>(&mut self) -> &mut Self {
        self.component_maps.insert(EntityMap::<C>::default());

        self
    }

    pub fn create(&mut self) -> Result<VersionedIndex, EMError> {
        let entity = self.entity_allocator.allocate();

        self.entities.push(entity);

        Ok(entity)
    }

    pub fn delete(&mut self, entity: VersionedIndex) {
        self.entity_allocator.deallocate(entity);
    }

    pub fn add<C: Component + PartialEq + 'static>(
        &mut self,
        entity: &VersionedIndex,
        component: C
    ) {
        match self.component_maps.get_mut::<EntityMap<C>>() {
            None => (),
            Some(cmp_map) => {
                cmp_map.set(&entity, component)
            }
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
            None => None,
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
            None => None,
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

mod tests;
