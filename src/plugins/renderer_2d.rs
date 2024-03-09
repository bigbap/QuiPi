use super::Plugin;

pub struct Renderer2DPlugin {}

impl Plugin for Renderer2DPlugin {
    fn build(&self, app: &mut crate::prelude::App) -> crate::QPResult<()> {
        Ok(())
    }
}
