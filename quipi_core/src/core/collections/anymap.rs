use std::{any::TypeId, collections::HashMap};

#[derive(Debug)]
pub struct AnyMap(HashMap<TypeId, Box<dyn std::any::Any>>);

impl AnyMap {
    pub fn new() -> Self {
        Self(HashMap::<TypeId, Box<dyn std::any::Any>>::new())
    }

    pub fn insert<C: 'static>(&mut self, item: C) {
        self.0.insert(TypeId::of::<C>(), Box::new(item));
    }

    pub fn get<C: 'static>(&self) -> Option<&C> {
        self.0.get(&TypeId::of::<C>())
            .map(|any| any.downcast_ref::<C>()).unwrap_or(None)
    }

    pub fn get_mut<C: 'static>(&mut self) -> Option<&mut C> {
        self.0.get_mut(&TypeId::of::<C>())
            .map(|any| any.downcast_mut::<C>()).unwrap_or(None)
    }

    pub fn all_collections(&mut self) -> Vec<&Box<dyn std::any::Any>> {
        self.0.values().into_iter().collect()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}