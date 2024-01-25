use std::fmt::Debug;
use crate::{Component, VersionedIndex, Registry};

pub fn s_print<C: Component + Debug + 'static>(
    entity: &VersionedIndex,
    registry: &Registry
) {
    if let Some(cmp) = registry.get_component::<C>(entity) {
        println!("{:?}", cmp);
    }
}
