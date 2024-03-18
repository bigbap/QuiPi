use crate::{
    prelude::{UnsafeWorldCell, World},
    resources::Resource,
};

pub unsafe trait SystemParam {
    type Item<'world>: SystemParam;

    unsafe fn get_param<'world>(world: UnsafeWorldCell<'world>) -> Self::Item<'world>;
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
tuple_impl!(P0, P1, P2, P3, P4, P5, P6, P7);
tuple_impl!(P0, P1, P2, P3, P4, P5, P6, P7, P8);
tuple_impl!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9);
tuple_impl!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10);
tuple_impl!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11);
tuple_impl!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12);

unsafe impl<'a> SystemParam for &'a mut World {
    type Item<'w> = &'w mut World;

    #[inline]
    unsafe fn get_param<'w>(world: UnsafeWorldCell<'w>) -> Self::Item<'w> {
        world.world_mut()
    }
}

// Resource Params

pub type Res<'a, R> = Option<&'a R>;
pub type ResMut<'a, R> = Option<&'a mut R>;

unsafe impl<'a, R: Resource + 'static> SystemParam for Res<'a, R> {
    type Item<'w> = Res<'w, R>;

    #[inline]
    unsafe fn get_param<'w>(world: UnsafeWorldCell<'w>) -> Self::Item<'w> {
        world.world().resource::<R>()
    }
}

unsafe impl<'a, R: Resource + 'static> SystemParam for ResMut<'a, R> {
    type Item<'w> = ResMut<'w, R>;

    #[inline]
    unsafe fn get_param<'w>(world: UnsafeWorldCell<'w>) -> Self::Item<'w> {
        world.world_mut().resource_mut::<R>()
    }
}
