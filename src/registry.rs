use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use crate::{
    asset_manager::AssetManager,
    prelude::{qp_core::StringInterner, qp_ecs::EntityManager},
    resource_manager::ResourceManager,
};

/**
 * Anything that must be global to the aplication is can be accessed
 * from here. This should be the only thing that is passed around.
 */
pub struct GlobalRegistry {
    strings: Rc<RefCell<StringInterner>>,

    pub entities: EntityManager,
    pub assets: AssetManager,
    pub resources: ResourceManager,

    pub quit: bool,
}

impl GlobalRegistry {
    pub fn init() -> Self {
        let strings = Rc::from(RefCell::from(StringInterner::new()));
        let entities = EntityManager::new();
        let assets = AssetManager::new(Rc::downgrade(&strings));
        let resources = ResourceManager::new(Rc::downgrade(&strings));

        Self {
            entities,
            assets,
            strings,
            resources,
            quit: false,
        }
    }

    pub fn strings(&self) -> Ref<StringInterner> {
        self.strings.borrow()
    }

    pub fn strings_mut(&self) -> RefMut<StringInterner> {
        self.strings.borrow_mut()
    }

    pub fn flush(&mut self) {
        self.entities.flush();
        self.assets.flush();
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::qp_ecs::*;

    use super::*;

    #[derive(Component, Debug, PartialEq)]
    struct DrawComponent {
        shader_id: Option<u32>,
    }

    #[derive(Debug, Component, PartialEq, Default)]
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
