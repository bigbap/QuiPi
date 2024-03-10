use crate::{
    world::{RenderSchedule, StartupSchedule, UpdateSchedule},
    QPResult,
};

use super::Plugin;

pub struct SchedulesPlugin {}

impl Plugin for SchedulesPlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        app.world.add_schedule::<StartupSchedule>();
        app.world.add_schedule::<UpdateSchedule>();
        app.world.add_schedule::<RenderSchedule>();

        Ok(())
    }
}
