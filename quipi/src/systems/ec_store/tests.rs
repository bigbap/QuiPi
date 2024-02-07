#[cfg(test)]
mod ecs_tests {
    use super::super::*;

    #[derive(Component, PartialEq)]
    struct DrawComponent {}

    #[derive(Component, PartialEq)]
    struct TransformComponent {}

    #[test]
    fn ecs_register_component() {
        let mut registry = EntityManager::new().unwrap();
        assert_eq!(registry.registered_components_len(), 0);

        registry.register_component::<DrawComponent>();
        assert_eq!(registry.registered_components_len(), 1);

        registry.register_component::<TransformComponent>();
        assert_eq!(registry.registered_components_len(), 2);
    }
}

