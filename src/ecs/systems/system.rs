use std::{any::TypeId, hash::Hash};

use crate::prelude::{UnsafeWorldCell, World};

pub trait SystemId: Hash + Eq + PartialEq {
    fn id(&self) -> TypeId;
}

pub type BoxedSystem<Out = ()> = Box<dyn System<Out = Out>>;

pub trait System: 'static {
    type Out;

    fn run_unsafe(&mut self, world: UnsafeWorldCell) -> Self::Out;

    fn run(&mut self, world: &mut World) -> Self::Out {
        let world = world.as_unsafe_cell_mut();

        self.run_unsafe(world)
    }
}

pub trait IntoSystem<Out, Marker> {
    type System: System<Out = Out>;

    fn into_system(this: Self) -> Self::System;
}

// All systems implicitly implement IntoSystem
impl<T: System> IntoSystem<T::Out, ()> for T {
    type System = T;

    fn into_system(this: Self) -> Self {
        this
    }
}
