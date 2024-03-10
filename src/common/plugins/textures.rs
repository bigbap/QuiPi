use crate::{
    common::{assets::TextureAsset, resources::AssetStore},
    QPResult,
};

use super::Plugin;

#[derive(Default)]
pub struct TexturesPlugin {}

impl Plugin for TexturesPlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        app.world
            .registry
            .resources
            .add_resource(AssetStore::<TextureAsset>::new())?;

        Ok(())
    }
}
