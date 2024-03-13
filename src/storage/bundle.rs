use std::{any::TypeId, cell::RefCell, collections::HashMap, rc::Weak};

use super::prelude::{Allocator, Index, IndexedArray};

pub trait Bundle {
    fn add_components(
        self,
        component_map: &mut ComponentMap,
        allocator: Weak<RefCell<Allocator>>,
        entity: &Index,
    );
}

#[derive(Default)]
pub struct BundleBuilder {
    bundle_loaders: Vec<Box<dyn FnMut(&mut ComponentMap, Weak<RefCell<Allocator>>, &Index)>>,
}

impl BundleBuilder {
    pub fn add_bundle(&mut self, bundle: impl Bundle + Clone + 'static) -> &mut Self {
        self.bundle_loaders.push(Box::new(
            move |component_map: &mut ComponentMap,
                  allocator: Weak<RefCell<Allocator>>,
                  entity: &Index| {
                bundle
                    .clone()
                    .add_components(component_map, allocator, entity)
            },
        ));

        self
    }
}

impl Bundle for BundleBuilder {
    fn add_components(
        mut self,
        component_map: &mut ComponentMap,
        allocator: Weak<RefCell<Allocator>>,
        entity: &Index,
    ) {
        for bundle_loader in self.bundle_loaders.iter_mut() {
            bundle_loader(component_map, allocator.clone(), entity);
        }
    }
}

pub trait Component: Bundle + Clone + PartialEq {
    fn id() -> ComponentId
    where
        Self: Sized;
}

impl<C: Component + 'static> Bundle for C {
    fn add_components(
        self,
        component_map: &mut ComponentMap,
        allocator: Weak<RefCell<Allocator>>,
        entity: &Index,
    ) {
        let Some(allocator) = allocator.upgrade() else {
            println!("failed to upgrade allocator");

            return;
        };

        let id = Self::id();
        if !component_map.has_key(id) {
            component_map.insert(id, IndexedArray::<C>::new(allocator))
        }

        match component_map.get_mut::<IndexedArray<C>>(id) {
            None => {
                #[cfg(debug_assertions)]
                println!(
                    "component {:?} has not been registered",
                    std::any::type_name::<C>()
                );
            }
            Some(cmp_map) => {
                cmp_map.set(&entity, self);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ComponentId(TypeId);
impl ComponentId {
    pub fn new<C: Component + 'static>() -> Self {
        let type_id = TypeId::of::<C>();

        Self(type_id)
    }
}

#[derive(Debug)]
pub struct ComponentMap(HashMap<ComponentId, Box<dyn std::any::Any>>);

impl ComponentMap {
    pub fn new() -> Self {
        Self(HashMap::<ComponentId, Box<dyn std::any::Any>>::new())
    }

    pub fn insert<C: 'static>(&mut self, id: ComponentId, item: C) {
        self.0.insert(id, Box::new(item));
    }

    pub fn get<C: 'static>(&self, id: ComponentId) -> Option<&C> {
        self.0
            .get(&id)
            .map(|any| any.downcast_ref::<C>())
            .unwrap_or(None)
    }

    pub fn get_mut<C: 'static>(&mut self, id: ComponentId) -> Option<&mut C> {
        self.0
            .get_mut(&id)
            .map(|any| any.downcast_mut::<C>())
            .unwrap_or(None)
    }

    pub fn has_key(&self, id: ComponentId) -> bool {
        self.0.contains_key(&id)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
