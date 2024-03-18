use quipi::{plugin::Plugin, QPResult};

pub struct Stars;

impl Plugin for Stars {
    fn build(&self, app: &mut quipi::prelude::App) -> QPResult<()> {
        Ok(())
    }
}
