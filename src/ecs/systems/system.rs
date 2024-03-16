use std::{any::TypeId, hash::Hash};

use crate::{prelude::World, QPResult};

pub trait SystemId: Hash + Eq + PartialEq {
    fn id(&self) -> TypeId;
}

pub type BoxedSystem = Box<dyn System>;

pub trait System {
    fn run(&mut self, world: &mut World) -> QPResult<()>;
}

pub trait IntoSystem<Params> {
    type System: System + 'static;

    fn into_system(self) -> Self::System;
}

pub trait SystemParam {
    fn dummy() -> Self;
}
impl SystemParam for () {
    fn dummy() -> Self {}
}

macro_rules! tuple_impl {
    ($($P: ident),*) => {
        #[allow(non_snake_case)]
        impl<$($P: SystemParam),*> SystemParam for ($($P,)*) {
            #[inline(always)]
            fn dummy() -> Self {
                ($($P::dummy(),)*)
            }
        }
    }
}

tuple_impl!(P0);
tuple_impl!(P0, P1);
tuple_impl!(P0, P1, P2);
tuple_impl!(P0, P1, P2, P3);
tuple_impl!(P0, P1, P2, P3, P4);
tuple_impl!(P0, P1, P2, P3, P4, P5);
tuple_impl!(P0, P1, P2, P3, P4, P5, P6);
