use crate::{prelude::App, QPResult};

pub trait Plugin: Plugins {
    /// runs first
    fn build(&self, app: &mut App) -> QPResult<()>;

    /// runs after all plugins finish running build()
    fn done(&self, _app: &mut App) -> QPResult<()> {
        Ok(())
    }

    /// runs after all plugins finish running done()
    fn cleanup(&self, _app: &mut App) -> QPResult<()> {
        Ok(())
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

pub trait Plugins {
    fn install(self, app: &mut App);
}

impl<P: Plugin + 'static> Plugins for P {
    fn install(self, app: &mut App) {
        app.add_plugin(Box::new(self))
    }
}

macro_rules! tuple_impl {
    ($($name: ident),*) => {
        #[allow(non_snake_case)]
        impl<$($name: Plugins),*> Plugins for ($($name,)*) {
            #[inline(always)]
            fn install(
                self,
                _app: &mut App
            ) {
                let ($($name,)*) = self;

                $($name.install(_app);)*
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
