use crate::prelude::{qp_ecs::Component, GlobalRegistry, Index};

pub struct EMQuery<A, B = (), C = ()>
where
    A: Component + PartialEq + 'static,
    B: Component + PartialEq + 'static,
    C: Component + PartialEq + 'static,
{
    _marker_a: ::std::marker::PhantomData<A>,
    _marker_b: ::std::marker::PhantomData<B>,
    _marker_c: ::std::marker::PhantomData<C>,
}

impl<A, B, C> EMQuery<A, B, C>
where
    A: Component + PartialEq + 'static,
    B: Component + PartialEq + 'static,
    C: Component + PartialEq + 'static,
{
    pub fn query_all(registry: &GlobalRegistry) -> Vec<Index> {
        let entities_a = registry.entity_manager.query_all::<A>();

        let entities = match std::any::type_name::<B>() {
            "()" => entities_a,
            _ => {
                let mut entities_b = vec![];
                for entity in entities_a {
                    if registry.entity_manager.get::<B>(&entity).is_some() {
                        entities_b.push(entity);
                    }
                }

                entities_b
            }
        };

        match std::any::type_name::<C>() {
            "()" => entities,
            _ => {
                let mut entities_c = vec![];
                for entity in entities {
                    if registry.entity_manager.get::<C>(&entity).is_some() {
                        entities_c.push(entity);
                    }
                }

                entities_c
            }
        }
    }
}
