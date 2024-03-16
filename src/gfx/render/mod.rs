pub mod assets;
pub mod batch;
pub mod cameras;
pub mod pipeline;
pub mod texture;
pub mod viewport;

use assets::*;

use crate::{
    assets::AssetServer,
    plugin::Plugin,
    prelude::{QPError, RenderSchedule, StorageId::*, StorageManager},
    schedule::ScheduleManager,
    QPResult,
};

#[derive(Default)]
pub struct RenderBasePlugin {}

impl Plugin for RenderBasePlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        let schedules = app
            .world
            .resource_mut::<ScheduleManager>()
            .ok_or(QPError::ResourceNotFound("ScheduleManager".into()))?;
        schedules.add_schedule::<RenderSchedule>();

        app.add_resource(AssetServer::new())
            .init_asset_store::<Texture>()
            .init_asset_store::<Shader>();

        let storage_manager = app
            .world
            .resources
            .get_mut::<StorageManager>()
            .expect("storage manager resource not loaded");
        storage_manager.insert(Cameras)?;

        app.add_system::<RenderSchedule>(pipeline::start_render_pipeline);

        Ok(())
    }
}
