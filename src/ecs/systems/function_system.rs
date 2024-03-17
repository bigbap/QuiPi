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

impl<Out, Func: 'static> SystemParamFunction<fn() -> Out> for Func
where
    for<'a> &'a mut Func: FnMut() -> Out + FnMut(SystemParamItem<()>) -> Out,
    Out: 'static,
{
    type Out = Out;
    type Param = ();

    fn run(&mut self, _param_value: SystemParamItem<Self::Param>) -> Self::Out {
        fn call_inner<Out>(mut f: impl FnMut() -> Out, _: ()) -> Out {
            f()
        }

        call_inner(self, ())
    }
}

impl<Out, Func: 'static, P0: SystemParam> SystemParamFunction<fn(P0) -> Out> for Func
where
    for<'a> &'a mut Func: FnMut(P0) -> Out + FnMut(SystemParamItem<P0>) -> Out,
    Out: 'static,
{
    type Out = Out;
    type Param = P0;

    fn run(&mut self, param_value: SystemParamItem<P0>) -> Self::Out {
        fn call_inner<Out, P0>(mut f: impl FnMut(P0) -> Out, p0: P0) -> Out {
            f(p0)
        }

        let p0 = param_value;
        call_inner(self, p0)
    }
}

// macro_rules! tuple_impl {
//     ($Id: ident, $(($G: ident, $Res: ident)),*) => {
//         #[allow(non_snake_case)]
//         impl<$($G: Bundle),*> Bundle for ($($G,)*) {
//             #[inline(always)]
//             fn add_components(
//                 self,
//                 _component_map: &mut ComponentMap,
//                 _allocator: Weak<RefCell<Allocator>>,
//                 _entity: &Index
//             ) {
//                 let ($($G,)*) = self;

//                 $($G.add_components(_component_map, _allocator.clone(), _entity);)*
//             }
//         }
//     }
// }
