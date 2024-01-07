use crate::ecs;
use std::{
    rc::Rc,
    cell::RefCell
};

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("there was a problem initializing the registry")]
    ProblemInitialisingRegistry(
        #[from]
        ecs::ECSError
    )
}

#[derive(Debug)]
pub struct Registry {
    pub components: Rc<RefCell<ecs::ComponentRegistry>>,
    pub resources: Rc<RefCell<ecs::ComponentRegistry>>
}

impl Registry {
    pub fn init() -> Result<Self, RegistryError> {
        let components = ecs::ComponentRegistry::new()?;
        let resources = ecs::ComponentRegistry::new()?;

        Ok(Self {
            components: Rc::new(RefCell::new(components)),
            resources: Rc::new(RefCell::new(resources))
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::ecs::Component;

//     use super::*;

//     #[derive(Component, Debug, PartialEq)]
//     struct DrawComponent {
//         shader_id: Option<u32>
//     }

//     #[derive(Debug, Component, PartialEq, Default)]
//     struct TransformComponent {
//         translate: glm::Vec3,
//         scale: glm::Vec3,
//         rotate: glm::Vec3
//     }

//     fn create_registry() -> Registry {
//         let mut registry = Registry::init().unwrap();

//         registry.components.register_component::<DrawComponent>();
//         registry.components.register_component::<TransformComponent>();

//         registry
//     }

//     #[test]
//     fn registry_create_entities() {
//         let mut registry = create_registry();

//         let player = registry.components.create_entity().unwrap();
//         registry.components.add_component(&player, DrawComponent { shader_id: Some(1234) });
//         registry.components.add_component(&player, TransformComponent {
//             translate: glm::vec3(1.0, 1.0, 1.0),
//             ..TransformComponent::default()
//         });

//         assert_eq!(
//             *registry.components.get_component::<DrawComponent>(&player).unwrap(),
//             DrawComponent { shader_id: Some(1234) }
//         );
//         assert_eq!(
//             *registry.components.get_component::<TransformComponent>(&player).unwrap(),
//             TransformComponent { translate: glm::vec3(1.0, 1.0, 1.0), ..TransformComponent::default() }
//         );
//     }
// }
