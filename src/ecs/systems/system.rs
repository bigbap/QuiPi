use std::{any::TypeId, hash::Hash};

use crate::{
    prelude::{UnsafeWorldCell, World},
    QPResult,
};

pub trait SystemId: Hash + Eq + PartialEq {
    fn id(&self) -> TypeId;
}

pub type BoxedSystem = Box<dyn System<Out = QPResult<()>>>;

pub trait System: 'static {
    type Out;

    fn run_unsafe(&mut self, world: UnsafeWorldCell) -> Self::Out;

    fn run(&mut self, world: &mut World) -> Self::Out {
        let world = world.as_unsafe_cell();

        self.run_unsafe(world)
    }
}

pub trait IntoSystem<Out, Marker> {
    type System: System<Out = Out>;

    fn into_system(this: Self) -> Self::System;
}

pub unsafe trait SystemParam {
    type Item<'world>: SystemParam;

    unsafe fn get_param<'world>(world: UnsafeWorldCell<'world>) -> Self::Item<'world>;
}

// All systems implicitly implement IntoSystem
impl<T: System> IntoSystem<T::Out, ()> for T {
    type System = T;

    fn into_system(this: Self) -> Self {
        this
    }
}

pub type SystemParamItem<'w, P> = <P as SystemParam>::Item<'w>;

macro_rules! tuple_impl {
    ($($Param: ident),*) => {
        #[allow(non_snake_case, unused)]
        unsafe impl<$($Param: SystemParam),*> SystemParam for ($($Param,)*) {
            type Item<'w> = ($($Param::Item::<'w>,)*);

            #[inline(always)]
            unsafe fn get_param<'w>(world: UnsafeWorldCell<'w>) -> Self::Item<'w> {
                ($($Param::get_param(world),)*)
            }
        }
    }
}

tuple_impl!();
tuple_impl!(P0);
tuple_impl!(P0, P1);
tuple_impl!(P0, P1, P2);
tuple_impl!(P0, P1, P2, P3);
tuple_impl!(P0, P1, P2, P3, P4);
tuple_impl!(P0, P1, P2, P3, P4, P5);
tuple_impl!(P0, P1, P2, P3, P4, P5, P6);

unsafe impl<'a> SystemParam for &'a World {
    type Item<'w> = &'w World;

    #[inline]
    unsafe fn get_param<'w>(world: UnsafeWorldCell<'w>) -> Self::Item<'w> {
        world.world()
    }
}
