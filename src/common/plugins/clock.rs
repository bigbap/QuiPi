use crate::{common::resources::clock::Clock, QPResult};

use super::Plugin;

#[derive(Default)]
pub struct ClockPlugin {}

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        app.world.registry.resources.add_resource(Clock::new())?;

        Ok(())
    }
}
