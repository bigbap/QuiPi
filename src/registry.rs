use std::{collections::HashMap, fmt::Debug};

use crate::prelude::{
    core::StringInterner,
    ecs::{
        ECSError,
        EntityManager,
        Component,
        VersionedIndex
    }
};

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("there was a problem initializing the registry")]
    ProblemInitialisingRegistry(
        #[from]
        ECSError
    ),

    #[error("there was a problem creating a new entity")]
    ProblemCreatingEntity,

    #[error("trying to load existing resource")]
    DuplicateResource
}

pub struct Registry {
    pub entities: EntityManager,
    resources: EntityManager,

    pub string_interner: StringInterner,
    index_map: HashMap<u64, VersionedIndex>
}

impl Registry {
    pub fn init() -> Result<Self, RegistryError> {
        let entities = EntityManager::new()?;
        let resources = EntityManager::new()?;

        Ok(Self {
            entities,
            resources,
            string_interner: StringInterner::new(),
            index_map: HashMap::new()
        })
    }

    pub fn register_resource<C: Component + std::fmt::Debug + PartialEq + 'static>(
        &mut self
    ) -> &mut Self {
        self.resources.register_component::<C>();

        self
    }

    pub fn load_resourse<C: Component + std::fmt::Debug + PartialEq + 'static>(
        &mut self,
        unique_name: String,
        resource: C
    ) -> Result<u64, RegistryError> {
        let id = self.string_interner.intern(unique_name);

        if self.index_map.get(&id).is_some() {
            return Err(RegistryError::DuplicateResource)
        }

        let index = self.resources.create();
        self.resources.add(&index, resource);

        self.index_map.insert(id, index);

        Ok(id)
    }

    pub fn get_resource<C: Component + std::fmt::Debug + PartialEq + 'static>(
        &self,
        id: u64
    ) -> Option<&C> {
        if let Some(index) = self.index_map.get(&id) {
            return self.resources.get::<C>(index)
        }

        None
    }

    pub fn get_resource_mut<C: Component + std::fmt::Debug + PartialEq + 'static>(
        &mut self,
        id: u64
    ) -> Option<&mut C> {
        if let Some(index) = self.index_map.get(&id) {
            return self.resources.get_mut::<C>(index)
        }

        None
    }

    pub fn get_resource_id(
        &mut self,
        id_as_str: &str
    ) -> Option<u64> {
        let id = self.string_interner.intern(id_as_str.to_string());

        match self.index_map.contains_key(&id) {
            true => Some(id),
            _ => None
        }
    }

    pub fn unload_resource<C: Component + std::fmt::Debug + PartialEq + 'static>(
        &mut self,
        id: u64
    ) {
        if let Some(index) = self.index_map.get(&id) {
            self.resources.remove::<C>(index);

            self.index_map.remove(&id);
        }
    }

    pub fn store_index(&mut self, id: u64, index: VersionedIndex) {
        self.index_map.insert(id, index);
    }

    pub fn lookup_index(&self, id: u64) -> Option<VersionedIndex> {
        self.index_map.get(&id).cloned()
    }

    pub fn flush_resources(&mut self) {
        self.resources.flush();
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::ecs::Component;

    use super::*;

    #[derive(Component, Debug, PartialEq)]
    struct DrawComponent {
        shader_id: Option<u32>
    }

    #[derive(Debug, Component, PartialEq, Default)]
    struct TransformComponent {
        translate: glm::Vec3,
        scale: glm::Vec3,
        rotate: glm::Vec3
    }

    fn create_registry() -> Registry {
        let mut registry = Registry::init().unwrap();

        registry.entities
            .register_component::<DrawComponent>()
            .register_component::<TransformComponent>();

        registry
    }

    #[test]
    fn registry_create_entities() {
        let mut registry = create_registry();

        let player = registry.entities.create();
        registry.entities.add(&player, DrawComponent { shader_id: Some(1234) });
        registry.entities.add(&player, TransformComponent {
            translate: glm::vec3(1.0, 1.0, 1.0),
            ..TransformComponent::default()
        });

        assert_eq!(
            *registry.entities.get::<DrawComponent>(&player).unwrap(),
            DrawComponent { shader_id: Some(1234) }
        );
        assert_eq!(
            *registry.entities.get::<TransformComponent>(&player).unwrap(),
            TransformComponent { translate: glm::vec3(1.0, 1.0, 1.0), ..TransformComponent::default() }
        );
    }
}
