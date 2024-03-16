pub use macros::Resource;

use std::{any::TypeId, collections::HashMap};

use crate::{prelude::QPError, QPResult};

pub struct ResourceManager {
    resources: HashMap<TypeId, ResourceOwner>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub(crate) fn insert_owner(&mut self, resource: ResourceOwner) -> QPResult<()> {
        let id = resource.id();
        if self.resources.get(&id).is_some() {
            return Err(QPError::DuplicateResource);
        }

        self.resources.insert(id, resource);

        Ok(())
    }

    pub fn insert<R: Resource + 'static>(&mut self, resource: R) -> QPResult<()> {
        let id = self.id::<R>();
        if self.resources.get(&id).is_some() {
            return Err(QPError::DuplicateResource);
        }

        self.resources
            .insert(id, ResourceOwner(Box::new(resource), id));

        Ok(())
    }

    pub fn remove<R: Resource + 'static>(&mut self) -> Option<ResourceOwner> {
        let id = self.id::<R>();

        self.resources.remove(&id)
    }

    pub fn remove_or_err<R: Resource + 'static>(&mut self) -> QPResult<ResourceOwner> {
        let id = self.id::<R>();

        self.resources
            .remove(&id)
            .ok_or(QPError::ResourceNotFound(resource_name::<R>()))
    }

    pub fn get<R: Resource + 'static>(&self) -> Option<&R> {
        match self.resources.get(&self.id::<R>()) {
            Some(resource) => resource.borrow(),
            _ => None,
        }
    }

    pub fn get_mut<R: Resource + 'static>(&mut self) -> Option<&mut R> {
        match self.resources.get_mut(&self.id::<R>()) {
            Some(resource) => resource.borrow_mut(),
            _ => None,
        }
    }

    fn id<R: Resource + 'static>(&self) -> TypeId {
        std::any::TypeId::of::<R>()
    }
}

pub trait Resource: AsAny {
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

pub use crate::core::prelude::AsAny;

fn resource_name<R: Resource>() -> String {
    std::any::type_name::<R>().into()
}

pub struct ResourceOwner(pub Box<dyn Resource>, TypeId);
impl ResourceOwner {
    pub fn borrow<R: Resource + 'static>(&self) -> Option<&R> {
        self.0.as_any().downcast_ref::<R>()
    }

    pub fn borrow_mut<R: Resource + 'static>(&mut self) -> Option<&mut R> {
        self.0.as_any_mut().downcast_mut::<R>()
    }

    pub fn id(&self) -> TypeId {
        self.1
    }
}
