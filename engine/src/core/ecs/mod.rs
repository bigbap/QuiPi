mod tests;

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
pub enum ECSError {
    #[error("there was a problem creating a new component registry")]
    ProblemCreatingNewComponentRegistry
}

#[derive(Debug)]
pub struct ECS {
    entity_allocator: VersionedIndexAllocator,
    component_maps: AnyMap
}

impl ECS {
    pub fn new() -> Result<Self, ECSError> {
        Ok(Self {
            entity_allocator: VersionedIndexAllocator::default(),
            component_maps: AnyMap::new()
        })
    }

    pub fn register_component<C: Component + 'static>(&mut self) {
        self.component_maps.insert(EntityMap::<C>::default());
    }

    pub fn create_entity(&mut self) -> Result<VersionedIndex, ECSError> {
        Ok(self.entity_allocator.allocate())
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

    pub fn get_component<C: 'static>(
        &self,
        entity: &VersionedIndex
    ) -> Option<&C> {
        match self.component_maps.get::<EntityMap<C>>() {
            None => None,
            Some(cmp_map) => match cmp_map.get(entity) {
                None => None,
                Some(cmp) => Some(cmp)
            }
        }
    }

    pub fn get_component_mut<C: 'static>(
        &mut self,
        entity: &VersionedIndex
    ) -> Option<&mut C> {
        match self.component_maps.get_mut::<EntityMap<C>>() {
            None => None,
            Some(cmp_map) => match cmp_map.get_mut(entity) {
                None => None,
                Some(cmp) => Some(cmp)
            }
        }
    }

    pub fn registered_components_len(&self) -> usize {
        self.component_maps.len()
    }
}
