use crate::prelude::UnsafeWorldCell;

use super::{IntoSystem, System, SystemParam, SystemParamItem};
use std::marker::PhantomData;

pub struct FunctionSystem<F, Marker>
where
    F: SystemParamFunction<Marker>,
{
    func: F,

    _marker: PhantomData<fn() -> Marker>,
}

impl<F, Marker> System for FunctionSystem<F, Marker>
where
    F: SystemParamFunction<Marker>,
    Marker: 'static,
{
    type Out = F::Out;

    fn run_unsafe(&mut self, world: UnsafeWorldCell) -> Self::Out {
        let params = unsafe { F::Param::get_param(world) };

        self.func.run(params)
    }
}

// marker to distiguish this from other systems
pub struct IsFunctionSystem;

impl<F, Marker> IntoSystem<F::Out, (IsFunctionSystem, Marker)> for F
where
    Marker: 'static,
    F: SystemParamFunction<Marker> + 'static,
{
    type System = FunctionSystem<F, Marker>;

    fn into_system(func: Self) -> Self::System {
        FunctionSystem {
            func,
            _marker: PhantomData,
        }
    }
}

pub trait SystemParamFunction<Marker>: 'static {
    type Out;

    type Param: SystemParam;

    fn run(&mut self, param_value: SystemParamItem<Self::Param>) -> Self::Out;
}

macro_rules! tuple_impl {
    ($($P: ident),*) => {
        #[allow(non_snake_case)]
        impl<Out, Func: 'static, $($P: SystemParam),*> SystemParamFunction<fn($($P,)*) -> Out> for Func
        where
            for<'a> &'a mut Func: FnMut($($P),*) -> Out + FnMut($(SystemParamItem<$P>),*) -> Out,
            Out: 'static,
        {
            type Out = Out;
            type Param = ($($P,)*);

            fn run(&mut self, param_value: SystemParamItem<($($P,)*)>) -> Self::Out {
                fn call_inner<Out, $($P,)*>(mut f: impl FnMut($($P,)*) -> Out, $($P: $P,)*) -> Out {
                    f($($P,)*)
                }

                let ($($P,)*) = param_value;
                call_inner(self, $($P),*)
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
