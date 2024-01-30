use crate::{
    EntityManager,
    ec_store::EMError,
    Component,
    VersionedIndex
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
    entities: EntityManager,
    resources: EntityManager,

    currently_building: Option<VersionedIndex>,
}

impl Registry {
    pub fn init() -> Result<Self, RegistryError> {
        let entities = EntityManager::new()?;
        let resources = EntityManager::new()?;

        Ok(Self {
            entities,
            resources,
            currently_building: None,
        })
    }

    pub fn entity_count(&self) -> usize {
        self.entities.count()
    }

    // entities

    pub fn create_entity(&mut self, tag: &str) -> Result<&mut Self, RegistryError> {
        self.currently_building = Some(self.entities.create_entity(tag)?);

        Ok(self)
    }

    pub fn with(
        &mut self,
        cmp: impl Component + 'static
    ) -> Result<&mut Self, Box<dyn std::error::Error>> {
        self.entities.add_component(&self.currently_building.unwrap(), cmp);

        Ok(self)
    }

    pub fn with_factory<C: Component + 'static>(
        &mut self,
        fac: impl Fn(&mut Self) -> Result<C, Box<dyn std::error::Error>>
    ) -> Result<&mut Self, Box<dyn std::error::Error>> {
        let cmp = fac(self)?;

        self.entities.add_component(&self.currently_building.unwrap(), cmp);

        Ok(self)
    }

    pub fn done(&mut self) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
        let entity = self.currently_building.unwrap();

        self.currently_building = None;

        Ok(entity)
    }

    pub fn get_entities_by_tag(&self, tag: &str) -> Vec<VersionedIndex> {
        self.entities.get_entities_by_tag(tag)
    }

    pub fn delete_entity(
        &mut self,
        entity: VersionedIndex
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.entities.delete_entity(entity);

        Ok(())
    }

    // components

    pub fn register_component<C: Component + 'static>(&mut self) -> &mut Self {
        self.entities.register_component::<C>();

        self
    }

    pub fn get_component_mut<C: Component + 'static>(&mut self, entity: &VersionedIndex) -> Option<&mut C> {
        self.entities.get_component_mut::<C>(entity)
    }

    pub fn get_component<C: Component + 'static>(&self, entity: &VersionedIndex) -> Option<&C> {
        self.entities.get_component::<C>(entity)
    }

    // resources

    pub fn register_resource<C: Component + 'static>(&mut self) -> &mut Self {
        self.resources.register_component::<C>();

        self
    }

    pub fn create_resource(&mut self, res: impl Component + 'static) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
        let resource = self.resources.create_entity("resource")?;

        self.resources.add_component(&resource, res);

        Ok(resource)
    }

    pub fn get_resource_mut<C: Component + 'static>(&mut self, entity: &VersionedIndex) -> Option<&mut C> {
        self.resources.get_component_mut::<C>(entity)
    }

    pub fn get_resource<C: Component + 'static>(&self, entity: &VersionedIndex) -> Option<&C> {
        self.resources.get_component::<C>(entity)
    }

    pub fn delete_resource(&mut self, resource: VersionedIndex) -> Result<(), Box<dyn std::error::Error>> {
        self.resources.delete_entity(resource);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Component;

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

        let player = registry.create_entity("player").unwrap()
            .with(DrawComponent { shader_id: Some(1234) }).unwrap()
            .with(TransformComponent {
                translate: glm::vec3(1.0, 1.0, 1.0),
                ..TransformComponent::default()
            }).unwrap()
            .done().unwrap();

        assert_eq!(
            *registry.entities.get_component::<DrawComponent>(&player).unwrap(),
            DrawComponent { shader_id: Some(1234) }
        );
        assert_eq!(
            *registry.entities.get_component::<TransformComponent>(&player).unwrap(),
            TransformComponent { translate: glm::vec3(1.0, 1.0, 1.0), ..TransformComponent::default() }
        );
    }
}
