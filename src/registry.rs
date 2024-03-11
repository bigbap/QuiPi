use crate::{prelude::qp_ecs::EntityManager, resources::ResourceManager};

/**
 * Anything that must be global to the aplication is can be accessed
 * from here. This should be the only thing that is passed around.
 */
pub struct GlobalRegistry {
    pub entities: EntityManager,
    pub resources: ResourceManager,
}

impl GlobalRegistry {
    pub fn init() -> Self {
        let entities = EntityManager::new();
        let resources = ResourceManager::new();

        Self {
            entities,
            resources,
        }
    }

    pub fn flush(&mut self) {
        self.entities.flush();
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::qp_ecs::*;

    use super::*;

    #[derive(Component, Debug, PartialEq, Clone)]
    struct DrawComponent {
        shader_id: Option<u32>,
    }

    #[derive(Debug, Component, PartialEq, Default, Clone)]
    struct TransformComponent {
        translate: glm::Vec3,
        scale: glm::Vec3,
        rotate: glm::Vec3,
    }

    fn create_registry() -> GlobalRegistry {
        GlobalRegistry::init()
    }

    #[test]
    fn registry_create_entities() {
        let mut registry = create_registry();

        let player = registry.entities.create((
            DrawComponent {
                shader_id: Some(1234),
            },
            TransformComponent {
                translate: glm::vec3(1.0, 1.0, 1.0),
                ..TransformComponent::default()
            },
        ));

        assert_eq!(
            *registry.entities.get::<DrawComponent>(&player).unwrap(),
            DrawComponent {
                shader_id: Some(1234)
            }
        );
        assert_eq!(
            *registry
                .entities
                .get::<TransformComponent>(&player)
                .unwrap(),
            TransformComponent {
                translate: glm::vec3(1.0, 1.0, 1.0),
                ..TransformComponent::default()
            }
        );
    }
}
