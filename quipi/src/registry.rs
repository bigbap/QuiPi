use crate::{
    EntityManager,
    ec_store::EMError
};

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("there was a problem initializing the registry")]
    ProblemInitialisingRegistry(
        #[from]
        EMError
    ),

    #[error("there was a problem creating a new entity")]
    ProblemCreatingEntity
}

#[derive(Debug)]
pub struct Registry {
    pub entities: EntityManager,
    pub resources: EntityManager,
}

impl Registry {
    pub fn init() -> Result<Self, RegistryError> {
        let entities = EntityManager::new()?;
        let resources = EntityManager::new()?;

        Ok(Self {
            entities,
            resources,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Component;

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

        let player = registry.entities.create().unwrap();
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
