use crate::{
    common::{assets::ShaderAsset, resources::AssetStore},
    QPResult,
};

use super::Plugin;

#[derive(Default)]
pub struct ShadersPlugin {}

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        app.world
            .registry
            .resources
            .add_resource(AssetStore::<ShaderAsset>::new())?;

        Ok(())
    }
}
