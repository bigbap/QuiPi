use crate::{
    assets::{AssetId, AssetServer, Assets},
    gfx::render::{Font, FontLoader},
    plugin::Plugin,
    prelude::qp_gfx::text::TEXT_SHADER_NAME,
};

pub struct RenderText {
    shader: AssetId,
    fonts: Vec<String>,
}

impl Plugin for RenderText {
    fn build(&self, app: &mut crate::prelude::App) -> crate::QPResult<()> {
        app.init_asset_store::<Font>();

        let asset_server = app.world.resource_mut::<AssetServer>().unwrap();
        let interner = app.world.interner_mut();
        let shader_id = AssetId::Id(interner.intern(TEXT_SHADER_NAME));

        for font in self.fonts.iter() {
            let font_id = interner.intern(&font);
            let loaded_font = asset_server.load(FontLoader { path: font })?;

            app.world
                .resource_mut::<Assets<Font>>()
                .unwrap()
                .add(font_id, loaded_font);
        }

        Ok(())
    }
}
