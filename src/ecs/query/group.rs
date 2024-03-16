use super::{super::prelude::Component, GroupIter};
use crate::ecs::storage::Storage;

pub trait Group {
    fn into_iter(storage: &'static Storage) -> Box<dyn GroupIter<Self>>;
}

impl<C0: Component> Group for C0 {
    fn into_iter(storage: &'static Storage) -> Box<dyn GroupIter<Self>> {
        Box::new(storage.iter::<C0>().unwrap())
    }
}

// impl<C0: Component, C1: Component> Group for (C0, C1) {
//     fn into_iter(storage: &'static Storage) -> Box<dyn GroupIter<Self>> {
//         Box::new((storage.iter::<C0>().unwrap(), storage.iter::<C1>().unwrap()))
//     }
// }

// pub trait Group {}

// impl<C0: Component> Group for C0 {}

// impl<C0: Component, C1: Component> Group for (C0, C1) {}

pub trait GroupResult {}

impl<C0: Component> GroupResult for &C0 {}
impl<C0: Component, C1: Component> GroupResult for (&C0, &C1) {}

#[cfg(test)]
mod tests {
    use crate::prelude::{StorageId, StorageManager};

    use super::*;

    #[derive(Component, Debug, PartialEq, Eq, Clone)]
    struct Container1(pub u32);

    #[derive(Component, Debug, PartialEq, Eq, Clone)]
    struct Container2(pub u32);

    // #[test]
    // fn with_storage() {
    //     let storage = Box::leak(Box::new(Storage::new()));

    //     let i = storage.create((Container1(123), Container2(456)));
    //     let mut iter = <(Container1, Container2) as Group>::into_iter(storage);
    //     let step = iter.step();

    //     assert_eq!(step, Some((i, (&Container1(123), &Container2(456)))));
    // }

    // #[test]
    // fn with_storage_manager() {
    //     let storage_manager = Box::leak(Box::new(StorageManager::new()));

    //     storage_manager
    //         .insert_storage_unit(StorageId::Entities)
    //         .unwrap();
    //     let id = storage_manager
    //         .create(StorageId::Entities, (&Container1(123), &Container2(456)))
    //         .unwrap();

    //     let mut query = storage_manager
    //         .query_v2::<(Container1, Container2)>(StorageId::Entities)
    //         .unwrap();

    //     let step = query.next();

    //     assert_eq!(step, Some((id, (Container1(123), Container2(456)))));
    // }
}
