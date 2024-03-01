use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use sdl2::event::Event;

use crate::{
    asset_manager::AssetManager,
    platform::sdl2::QPWindow,
    prelude::{
        qp_core::StringInterner,
        qp_ecs::EntityManager,
        qp_gfx::{Gfx, QPText},
    },
    QPResult,
};

/**
 * Anything that must be global to the aplication is can be accessed
 * from here. This should be the only thing that is passed around.
 */
pub struct GlobalRegistry {
    strings: Rc<RefCell<StringInterner>>,

    pub entity_manager: EntityManager,
    pub asset_manager: AssetManager,

    pub events: Vec<Event>,
    pub text_buffer: Vec<QPText>,
}

impl GlobalRegistry {
    pub fn init() -> QPResult<Self> {
        let strings = Rc::from(RefCell::from(StringInterner::new()));
        let entity_manager = EntityManager::new()?;
        let asset_manager = AssetManager::init(Rc::downgrade(&strings))?;

        Ok(Self {
            entity_manager,
            asset_manager,
            strings,
            events: vec![],
            text_buffer: vec![],
        })
    }

    pub fn strings(&self) -> Ref<StringInterner> {
        self.strings.borrow()
    }

    pub fn strings_mut(&self) -> RefMut<StringInterner> {
        self.strings.borrow_mut()
    }

    pub fn flush(&mut self) {
        self.entity_manager.flush();
        self.asset_manager.flush();
        self.text_buffer.clear();
    }

    pub fn new_frame(&mut self, winapi: &QPWindow) -> QPResult<()> {
        self.events = winapi.get_event_queue()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{qp_ecs::Component, qp_gfx::Viewport};

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
        let mut registry = GlobalRegistry::init(Gfx {
            viewport: Viewport::new(0, 0, 800, 600),
        })
        .unwrap();

        registry
            .entity_manager
            .register_component::<DrawComponent>()
            .register_component::<TransformComponent>();

        registry
    }

    #[test]
    fn registry_create_entities() {
        let mut registry = create_registry();

        let player = registry.entity_manager.create();
        registry.entity_manager.add(
            &player,
            DrawComponent {
                shader_id: Some(1234),
            },
        );
        registry.entity_manager.add(
            &player,
            TransformComponent {
                translate: glm::vec3(1.0, 1.0, 1.0),
                ..TransformComponent::default()
            },
        );

        assert_eq!(
            *registry
                .entity_manager
                .get::<DrawComponent>(&player)
                .unwrap(),
            DrawComponent {
                shader_id: Some(1234)
            }
        );
        assert_eq!(
            *registry
                .entity_manager
                .get::<TransformComponent>(&player)
                .unwrap(),
            TransformComponent {
                translate: glm::vec3(1.0, 1.0, 1.0),
                ..TransformComponent::default()
            }
        );
    }
}
