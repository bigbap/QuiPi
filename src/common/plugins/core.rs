use crate::{
    common::resources::{Clock, StringInterner},
    QPResult,
};

use super::Plugin;

pub(crate) struct CorePlugin {}

impl Plugin for CorePlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        app.add_resource(Clock::new());
        app.add_resource(StringInterner::new());

        Ok(())
    }
}
