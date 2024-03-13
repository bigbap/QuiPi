use crate::storage::indexed_array::*;
use crate::storage::prelude::*;
use std::{cell::RefCell, rc::Weak};

macro_rules! tuple_impl {
    ($Id: ident, $(($G: ident, $Res: ident)),*) => {
        #[allow(non_snake_case, unused_assignments)]
        impl<$($G: Component + 'static),*> GroupIter<($(&'static $G),*)> for ($(Iter<'static, $G>,)*) {
            #[inline(always)]
            fn step(&mut self) -> Option<(Index, ($(&'static $G),*))> {
                let ($($G,)*) = self;
                loop {
                    let mut $Id;
                    $(
                        let $Res = match $G.next() {
                            None => return None,
                            Some(Some((index, Some(entry)))) => {
                                $Id = index;
                                entry
                            },
                            _ => continue
                        };
                    )*

                    return Some(($Id, ($($Res,)*)))
                }
            }
        }

        #[allow(non_snake_case)]
        impl<$($G: Component + 'static),*> Group for ($(&'static $G,)*) {
            fn into_iter(storage: &'static Storage) -> Box<dyn GroupIter<Self>> {
                Box::new(($(
                    storage.query::<$G>().unwrap().iter(),
                )*))
            }
        }

        #[allow(non_snake_case)]
        impl<$($G: Bundle),*> Bundle for ($($G,)*) {
            #[inline(always)]
            fn add_components(
                self,
                _component_map: &mut ComponentMap,
                _allocator: Weak<RefCell<Allocator>>,
                _entity: &Index
            ) {
                let ($($G,)*) = self;

                $($G.add_components(_component_map, _allocator.clone(), _entity);)*
            }
        }
    }
}

tuple_impl!(Id, (G0, Res0), (G1, Res1));
tuple_impl!(Id, (G0, Res0), (G1, Res1), (G2, Res2));
tuple_impl!(Id, (G0, Res0), (G1, Res1), (G2, Res2), (G3, Res3));
tuple_impl!(
    Id,
    (G0, Res0),
    (G1, Res1),
    (G2, Res2),
    (G3, Res3),
    (G4, Res4)
);
tuple_impl!(
    Id,
    (G0, Res0),
    (G1, Res1),
    (G2, Res2),
    (G3, Res3),
    (G4, Res4),
    (G5, Res5)
);
tuple_impl!(
    Id,
    (G0, Res0),
    (G1, Res1),
    (G2, Res2),
    (G3, Res3),
    (G4, Res4),
    (G5, Res5),
    (G6, Res6)
);
tuple_impl!(
    Id,
    (G0, Res0),
    (G1, Res1),
    (G2, Res2),
    (G3, Res3),
    (G4, Res4),
    (G5, Res5),
    (G6, Res6),
    (G7, Res7)
);
tuple_impl!(
    Id,
    (G0, Res0),
    (G1, Res1),
    (G2, Res2),
    (G3, Res3),
    (G4, Res4),
    (G5, Res5),
    (G6, Res6),
    (G7, Res7),
    (G8, Res8)
);
tuple_impl!(
    Id,
    (G0, Res0),
    (G1, Res1),
    (G2, Res2),
    (G3, Res3),
    (G4, Res4),
    (G5, Res5),
    (G6, Res6),
    (G7, Res7),
    (G8, Res8),
    (G9, Res9)
);
tuple_impl!(
    Id,
    (G0, Res0),
    (G1, Res1),
    (G2, Res2),
    (G3, Res3),
    (G4, Res4),
    (G5, Res5),
    (G6, Res6),
    (G7, Res7),
    (G8, Res8),
    (G9, Res9),
    (G10, Res10)
);
