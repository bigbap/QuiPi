use crate::{common::resources::CameraList, QPResult};

use super::Plugin;

#[derive(Default)]
pub struct CamerasPlugin {}

impl Plugin for CamerasPlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        app.world
            .registry
            .resources
            .add_resource(CameraList::default())?;

        Ok(())
    }
}
