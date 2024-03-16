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

pub trait Component: Bundle + Clone + PartialEq + 'static {}

impl<C: Component> Bundle for C {
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

        if !component_map.has_key::<C>() {
            component_map.insert(IndexedArray::<C>::new(allocator))
        }

        match component_map.get_mut::<C>() {
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

// #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
// pub struct ComponentId(TypeId);
// impl ComponentId {
//     pub fn new<C: Component + 'static>() -> Self {
//         let type_id = TypeId::of::<C>();

//         Self(type_id)
//     }
// }

#[derive(Debug)]
pub struct ComponentMap(HashMap<TypeId, Box<dyn std::any::Any>>);

impl ComponentMap {
    pub(super) fn new() -> Self {
        Self(HashMap::<TypeId, Box<dyn std::any::Any>>::new())
    }

    pub(super) fn insert<C: Component>(&mut self, item: IndexedArray<C>) {
        self.0.insert(TypeId::of::<C>(), Box::new(item));
    }

    pub(super) fn get<C: Component>(&self) -> Option<&IndexedArray<C>> {
        self.0
            .get(&TypeId::of::<C>())
            .map(|any| any.downcast_ref::<IndexedArray<C>>())
            .unwrap_or(None)
    }

    pub(super) fn get_mut<C: Component>(&mut self) -> Option<&mut IndexedArray<C>> {
        self.0
            .get_mut(&TypeId::of::<C>())
            .map(|any| any.downcast_mut::<IndexedArray<C>>())
            .unwrap_or(None)
    }

    pub(super) fn has_key<C: Component>(&self) -> bool {
        self.0.contains_key(&TypeId::of::<C>())
    }

    pub(super) fn len(&self) -> usize {
        self.0.len()
    }
}
