use crate::{
    ecs,
    Component,
    VersionedIndex
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
    components: ecs::ComponentRegistry,
    resources: ecs::ComponentRegistry,

    currently_building: Option<VersionedIndex>,
}

impl Registry {
    pub fn init() -> Result<Self, RegistryError> {
        let components = ecs::ComponentRegistry::new()?;
        let resources = ecs::ComponentRegistry::new()?;

        Ok(Self {
            components,
            resources,
            currently_building: None,
        })
    }

    pub fn register_component<C: Component + 'static>(&mut self) -> &mut Self {
        self.components.register_component::<C>();

        self
    }

    pub fn register_resource<C: Component + 'static>(&mut self) -> &mut Self {
        self.resources.register_component::<C>();

        self
    }

    pub fn get_component_mut<C: Component + 'static>(&mut self, entity: &VersionedIndex) -> Option<&mut C> {
        self.components.get_component_mut::<C>(entity)
    }

    pub fn get_resource_mut<C: Component + 'static>(&mut self, entity: &VersionedIndex) -> Option<&mut C> {
        self.resources.get_component_mut::<C>(entity)
    }

    pub fn get_component<C: Component + 'static>(&self, entity: &VersionedIndex) -> Option<&C> {
        self.components.get_component::<C>(entity)
    }

    pub fn get_resource<C: Component + 'static>(&self, entity: &VersionedIndex) -> Option<&C> {
        self.resources.get_component::<C>(entity)
    }

    pub fn create_resource(&mut self, res: impl Component + 'static) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
        let resource = self.resources.create_entity()?;

        self.resources.add_component(&resource, res);

        Ok(resource)
    }

    pub fn create_entity(&mut self) -> &mut Self {
        self.currently_building = Some(self.components.create_entity().unwrap());

        self
    }

    pub fn with(&mut self, cmp: impl Component + 'static) -> &mut Self {
        self.components.add_component(&self.currently_building.unwrap(), cmp);

        self
    }

    pub fn done(&mut self) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
        let entity = self.currently_building.unwrap();

        self.currently_building = None;

        Ok(entity)
    }
}

#[cfg(test)]
mod tests {
    use super::ecs::Component;

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

        registry
            .register_component::<DrawComponent>()
            .register_component::<TransformComponent>();

        registry
    }

    #[test]
    fn registry_create_entities() {
        let mut registry = create_registry();

        let player = registry.create_entity()
            .with(DrawComponent { shader_id: Some(1234) })
            .with(TransformComponent {
                translate: glm::vec3(1.0, 1.0, 1.0),
                ..TransformComponent::default()
            }).done().unwrap();

        assert_eq!(
            *registry.components.get_component::<DrawComponent>(&player).unwrap(),
            DrawComponent { shader_id: Some(1234) }
        );
        assert_eq!(
            *registry.components.get_component::<TransformComponent>(&player).unwrap(),
            TransformComponent { translate: glm::vec3(1.0, 1.0, 1.0), ..TransformComponent::default() }
        );
    }
}
