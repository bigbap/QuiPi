pub mod indexed_array;

pub use indexed_array::VersionedIndex;
pub use indexed_array::VersionedIndexAllocator;
pub use indexed_array::IndexedArray;
pub use component_derive::Component;

pub trait Component {
    fn my_type(&self) -> String;
}

use anymap2::AnyMap;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Component, Clone, Serialize, Deserialize)]
pub struct CTag {
    pub tag: String
}

impl Default for CTag {
    fn default() -> Self {
        Self { tag: "default".to_string() }
    }
}

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

    entities: Vec<VersionedIndex>
}

impl EntityManager {
    pub fn new() -> Result<Self, EMError> {
        let mut entity_manager = Self {
            entity_allocator: VersionedIndexAllocator::default(),
            component_maps: AnyMap::new(),
            entities: Vec::<VersionedIndex>::new()
        };

        entity_manager.register_component::<CTag>();

        Ok(entity_manager)
    }

    pub fn register_component<C: Component + 'static>(&mut self) {
        self.component_maps.insert(EntityMap::<C>::default());
    }

    pub fn create_entity(&mut self, tag: &str) -> Result<VersionedIndex, EMError> {
        let entity = self.entity_allocator.allocate();

        self.add_component(&entity, CTag { tag: tag.into() });
        self.entities.push(entity);

        Ok(entity)
    }

    pub fn delete_entity(&mut self, entity: VersionedIndex) {
        self.entity_allocator.deallocate(entity);
    }

    pub fn add_component<C: Component + 'static>(
        &mut self,
        entity: &VersionedIndex,
        component: C
    ) {
        match self.component_maps.get_mut::<EntityMap<C>>() {
            None => (),
            Some(cmp_map) => {
                cmp_map.set(entity, component)
            }
        }
    }

    pub fn get_component<C: Component + 'static>(
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

    pub fn get_component_mut<C: Component + 'static>(
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

    pub fn get_entities_by_tag(
        &self,
        tag: &str
    ) -> Vec<VersionedIndex> {
        let Some(tag_map) = self.component_maps.get::<EntityMap<CTag>>() else { return vec![] };

        let mut result = Vec::<VersionedIndex>::new();
        for entity in &self.entities {
            if !self.entity_allocator.validate(&entity) {
                continue;
            }

            if let Some(c_tag) = tag_map.get(&entity) {
                if *c_tag.tag == *tag {
                    result.push(*entity);
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
            if self.entity_allocator.validate(&entity) {
                result.push(*entity);
            }
        }

        result
    }
}

mod tests;
