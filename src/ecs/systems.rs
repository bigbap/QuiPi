use std::any::TypeId;

pub trait SystemParams: Send + 'static {
    type Out;
    type Args;

    fn invoke(&mut self, args: Self::Args) -> Self::Out;
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct SystemId(pub TypeId);
impl SystemId {
    pub fn new<O, A, P: SystemParams<Out = O, Args = A>>() -> Self {
        Self(std::any::TypeId::of::<P>())
    }
}

// impl<F> System for F where F: FnMut(&mut Commands) -> QPResult<()> + 'static {}
// impl<G> System for G where G: FnMut(&mut World) -> QPResult<()> + 'static {}

// pub type BoxedSystem = Box<dyn System>;

// pub struct SystemExecuter {
//     system: BoxedSystem,
// }

// impl SystemExecuter {
//     pub fn new(system: BoxedSystem) -> SystemExecuter {
//         Self { system }
//     }

//     pub fn execute(&mut self, world: &mut World) -> QPResult<()> {
//         let mut commands = Commands::new();

//         (self.system)(&mut commands)
//     }
// }

pub trait System: Send + Sync + 'static {
    type Out;
}

pub trait IntoSystem<Out> {
    type System: System;
}
