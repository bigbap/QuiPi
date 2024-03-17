pub mod assets;
pub mod batch;
pub mod cameras;
pub mod pipeline;
pub mod renderers;
pub mod texture;
pub mod viewport;

use assets::*;

use crate::{
    plugin::Plugin,
    prelude::{QPError, StorageId::*, StorageManager},
    schedule::{Render, ScheduleManager},
    QPResult,
};

pub struct RenderBasePlugin;

impl Plugin for RenderBasePlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        let schedules = app
            .world
            .resource_mut::<ScheduleManager>()
            .ok_or(QPError::ResourceNotFound("ScheduleManager".into()))?;
        schedules.insert_schedule(Render);

        app.init_asset_store::<Texture>()
            .init_asset_store::<Shader>();

        let storage_manager = app
            .world
            .resources
            .get_mut::<StorageManager>()
            .expect("storage manager resource not loaded");
        storage_manager.insert(Cameras)?;

        app.add_system(Render, pipeline::start_render_pipeline);

        Ok(())
    }
}
