pub use macros::Resource;

use std::{any::TypeId, collections::HashMap};

use crate::{prelude::QPError, QPResult};

pub struct ResourceManager {
    resources: HashMap<TypeId, Box<dyn Resource + 'static>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn add_resource<R: Resource + 'static>(&mut self, resource: R) -> QPResult<()> {
        let id = self.id::<R>();
        if self.resources.get(&id).is_some() {
            return Err(QPError::DuplicateResource);
        }

        self.resources.insert(id, Box::new(resource));

        Ok(())
    }

    pub fn get<R: Resource + 'static>(&self) -> Option<&R> {
        match self.resources.get(&self.id::<R>()) {
            Some(resource) => resource.as_any().downcast_ref::<R>(),
            _ => None,
        }
    }

    pub fn get_mut<R: Resource + 'static>(&mut self) -> Option<&mut R> {
        match self.resources.get_mut(&self.id::<R>()) {
            Some(resource) => resource.as_any_mut().downcast_mut::<R>(),
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
