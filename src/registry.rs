use std::{cell::RefCell, rc::Rc};

use crate::{
    asset_manager::AssetManager,
    prelude::{
        qp_core::StringInterner,
        qp_ecs::EntityManager,
    },
    QPResult
};

/**
 * Anything that must be global to the aplication is can be accessed
 * from here. This should be the only thing that is passed around. 
 */
pub struct GlobalRegistry {
    pub string_interner: Rc<RefCell<StringInterner>>,
    pub entity_manager: EntityManager,
    pub asset_manager: AssetManager
}

impl GlobalRegistry {
    pub fn init() -> QPResult<Self> {
        let string_interner = Rc::new(RefCell::new(StringInterner::new()));

        // let mut string_interner = StringInterner::new();
        let entity_manager = EntityManager::new()?;
        let asset_manager = AssetManager::init(string_interner.clone())?;

        Ok(Self {
            entity_manager,
            asset_manager,
            string_interner
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::qp_ecs::Component;

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

    fn create_registry() -> GlobalRegistry {
        let mut registry = GlobalRegistry::init().unwrap();

        registry.entity_manager
            .register_component::<DrawComponent>()
            .register_component::<TransformComponent>();

        registry
    }

    #[test]
    fn registry_create_entities() {
        let mut registry = create_registry();

        let player = registry.entity_manager.create();
        registry.entity_manager.add(&player, DrawComponent { shader_id: Some(1234) });
        registry.entity_manager.add(&player, TransformComponent {
            translate: glm::vec3(1.0, 1.0, 1.0),
            ..TransformComponent::default()
        });

        assert_eq!(
            *registry.entity_manager.get::<DrawComponent>(&player).unwrap(),
            DrawComponent { shader_id: Some(1234) }
        );
        assert_eq!(
            *registry.entity_manager.get::<TransformComponent>(&player).unwrap(),
            TransformComponent {
                translate: glm::vec3(1.0, 1.0, 1.0),
                ..TransformComponent::default()
            }
        );
    }
}
