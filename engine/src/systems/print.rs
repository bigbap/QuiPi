use std::fmt::Debug;
use crate::{Component, VersionedIndex, Registry};

pub fn s_print<C: Component + Debug + 'static>(
    entity: &VersionedIndex,
    registry: &Registry
) {
    let cmp = registry.get_component::<C>(entity).unwrap();

    println!("{:?}", cmp);
}
