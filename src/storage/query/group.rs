use super::{super::prelude::Component, GroupIter};
use crate::storage::manager::Storage;

pub trait Group {
    fn into_iter(storage: &'static Storage) -> Box<dyn GroupIter<Self>>;
}

impl<C0: Component + 'static> Group for &'static C0 {
    fn into_iter(storage: &'static Storage) -> Box<dyn GroupIter<Self>> {
        Box::new(storage.query::<C0>().unwrap().iter())
    }
}

// impl<'a, C0: Component, C1: Component> Group<'a> for (&'a C0, &'a C1) {
//     fn into_iter(storage: &Storage) -> Box<dyn GroupIter<'a, Self>> {
//         Box::new((
//             storage.query::<C0>().unwrap().iter(),
//             storage.query::<C1>().unwrap().iter(),
//         ))
//     }
// }

macro_rules! tuple_impl {
    ($($G: ident,),*) => {
        #[allow(non_snake_case)]
        impl<'a, $($G: Component),*> Group<'a> for ($(&'a $G,)*) {
            fn into_iter(storage: &Storage) -> Box<dyn GroupIter<'a, Self>> {
                Box::new(($(
                    storage.query::<$G>().unwrap().iter(),
                )*))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::prelude::qp_storage::ComponentId;
    use crate::storage::indexed_array::{Allocator, IndexedArray};

    use super::super::GroupIter;
    use super::*;

    #[derive(Component, PartialEq, Clone)]
    struct C1 {}

    #[derive(Component, PartialEq, Clone)]
    struct C2 {}

    #[test]
    fn it_works() {
        #[derive(Component, Debug, PartialEq, Eq, Clone)]
        struct Container1(pub u32);

        #[derive(Component, Debug, PartialEq, Eq, Clone)]
        struct Container2(pub u32);

        let allocator = Rc::new(RefCell::new(Allocator::with_capacity(4)));
        let i = allocator.borrow_mut().allocate();

        let mut array1 = IndexedArray::<Container1>::new(allocator.clone());
        array1.set(&i, Container1(123));

        let mut array2 = IndexedArray::<Container2>::new(allocator.clone());
        array2.set(&i, Container2(456));

        let mut array3 = IndexedArray::<Container2>::new(allocator.clone());
        array3.set(&i, Container2(789));

        let mut iter = (array1.iter(), array2.iter());
        // let step = iter.step();

        // assert_eq!(step, Some((i, (&Container1(123), &Container2(456)))));
    }
}
