use crate::{prelude::World, QPResult};

use super::{IntoSystem, System, SystemParam};
use std::marker::PhantomData;

pub struct FunctionSystem<F, Params: SystemParam> {
    system: F,
    _marker: PhantomData<Params>,
}

impl<F, Params: SystemParam> System for FunctionSystem<F, Params>
where
    F: SystemParamFunction<Params>,
{
    fn run(&mut self, world: &mut World) -> QPResult<()> {
        SystemParamFunction::run(&mut self.system)
    }
}

impl<F, Params: SystemParam + 'static> IntoSystem<Params> for F
where
    F: SystemParamFunction<Params> + 'static,
{
    type System = FunctionSystem<F, Params>;

    fn into_system(self) -> Self::System {
        FunctionSystem {
            system: self,
            _marker: PhantomData,
        }
    }
}

pub trait SystemParamFunction<Params: SystemParam> {
    fn run(&mut self) -> QPResult<()>;
}
impl<F> SystemParamFunction<()> for F
where
    F: Fn() -> QPResult<()>,
{
    fn run(&mut self) -> QPResult<()> {
        self()
    }
}
impl<F, P0: SystemParam> SystemParamFunction<(P0,)> for F
where
    F: Fn(P0) -> QPResult<()>,
{
    fn run(&mut self) -> QPResult<()> {
        self(P0::dummy())
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
