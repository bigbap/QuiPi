use super::Plugin;
use crate::{
    common::{
        assets::{ShaderAsset, TextureAsset},
        resources::{AssetStore, CameraList},
    },
    prelude::RenderSchedule,
    storage::prelude::{StorageId::*, StorageManager},
    QPResult,
};

pub struct RenderBasePlugin {}

impl Plugin for RenderBasePlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        app.world.resources.add_resource(CameraList::default())?;

        app.world
            .resources
            .add_resource(AssetStore::<TextureAsset>::new())?;

        app.world
            .resources
            .add_resource(AssetStore::<ShaderAsset>::new())?;

        app.schedules.add_schedule::<RenderSchedule>();

        let storage_manager = app
            .world
            .resources
            .get_mut::<StorageManager>()
            .expect("storage manager resource not loaded");
        storage_manager.insert_storage_unit(Cameras);

        Ok(())
    }
}
