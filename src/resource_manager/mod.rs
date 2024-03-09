pub mod resources;

pub use macros::Resource;

use std::{
    any::Any,
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::{core::prelude::StringInterner, prelude::QPError, QPResult};

use self::resources::clock::Clock;

pub struct ResourceManager {
    resources: Vec<Box<dyn Resource + 'static>>,
    index_map: HashMap<u64, usize>,

    strings: Weak<RefCell<StringInterner>>,
}

impl ResourceManager {
    pub fn new(strings: Weak<RefCell<StringInterner>>) -> Self {
        let mut manager = Self {
            resources: vec![],
            index_map: HashMap::new(),
            strings,
        };

        manager.add_resource(Clock::new()).unwrap();

        manager
    }

    pub fn add_resource(&mut self, resource: impl Resource + 'static) -> QPResult<u64> {
        let name = resource.name();
        let Some(interner) = self.string_interner() else {
            return Err(QPError::SharedReferenceDropped);
        };

        let id = interner.borrow_mut().intern(name.into());

        if self.index_map.contains_key(&id) {
            return Err(QPError::Generic(
                "trying to add a duplicate resource".into(),
            ));
        }

        self.index_map.insert(id, self.resources.len());
        self.resources.push(Box::new(resource));

        Ok(id)
    }

    pub fn get<R: Resource + 'static>(&self, id: u64) -> Option<&R> {
        if let Some(index) = self.index_map.get(&id) {
            return match self.resources.get(*index) {
                Some(resource) => resource.as_any().downcast_ref::<R>(),
                _ => None,
            };
        }

        None
    }

    pub fn get_mut<R: Resource + 'static>(&mut self, id: u64) -> Option<&mut R> {
        if let Some(index) = self.index_map.get(&id) {
            return match self.resources.get_mut(*index) {
                Some(resource) => resource.as_any_mut().downcast_mut::<R>(),
                _ => None,
            };
        }

        None
    }

    fn string_interner(&mut self) -> Option<Rc<RefCell<StringInterner>>> {
        let Some(string_interner) = self.strings.upgrade() else {
            #[cfg(debug_assertions)]
            println!("[asset manager] weak reference to string_interner returned None");

            return None;
        };

        Some(string_interner)
    }

    pub fn resource_id<R: Resource>(&mut self) -> Option<u64> {
        let name = std::any::type_name::<R>();

        if let Some(interner) = self.string_interner() {
            return Some(interner.borrow_mut().intern(name.into()));
        }

        None
    }
}

pub trait Resource {
    fn name(&self) -> &str;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}
