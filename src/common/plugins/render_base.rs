use super::{cameras::CamerasPlugin, shaders::ShadersPlugin, textures::TexturesPlugin, Plugin};
use crate::QPResult;

pub struct RenderBasePlugin {}

impl Plugin for RenderBasePlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        app.add_plugins(ShadersPlugin::default())
            .add_plugins(TexturesPlugin::default())
            .add_plugins(CamerasPlugin::default());

        Ok(())
    }
}
