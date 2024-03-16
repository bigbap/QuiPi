use std::collections::HashMap;

pub use macros::AsAny;
pub trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

#[derive(Debug)]
pub struct AnyMap<Key: Eq + PartialEq + std::hash::Hash>(HashMap<Key, Box<dyn std::any::Any>>);

impl<Key: Eq + PartialEq + std::hash::Hash> AnyMap<Key> {
    pub fn new() -> Self {
        Self(HashMap::<Key, Box<dyn std::any::Any>>::new())
    }

    pub fn insert<C: 'static>(&mut self, key: Key, item: C) {
        self.0.insert(key, Box::new(item));
    }

    pub fn get<C: 'static>(&self, key: &Key) -> Option<&C> {
        self.0
            .get(key)
            .map(|any| any.downcast_ref::<C>())
            .unwrap_or(None)
    }

    pub fn get_mut<C: 'static>(&mut self, key: &Key) -> Option<&mut C> {
        self.0
            .get_mut(key)
            .map(|any| any.downcast_mut::<C>())
            .unwrap_or(None)
    }

    pub fn all_collections(&mut self) -> Vec<&Box<dyn std::any::Any>> {
        self.0.values().into_iter().collect()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
