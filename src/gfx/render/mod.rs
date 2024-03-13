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
    prelude::RenderSchedule,
    storage::prelude::{StorageId::*, StorageManager},
    QPResult,
};

#[derive(Default)]
pub struct RenderBasePlugin {}

impl Plugin for RenderBasePlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        app.schedules.add_schedule::<RenderSchedule>();
        app.add_resource(AssetServer::new())
            .init_asset_store::<Texture>()
            .init_asset_store::<Shader>();

        let storage_manager = app
            .world
            .resources
            .get_mut::<StorageManager>()
            .expect("storage manager resource not loaded");
        storage_manager.insert_storage_unit(Cameras)?;

        app.add_system::<RenderSchedule>(pipeline::start_render_pipeline);

        Ok(())
    }
}