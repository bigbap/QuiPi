use crate::{app::AppState, prelude::App, QPResult};

use super::Plugin;

pub struct MainLoopPlugin {}

impl Plugin for MainLoopPlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        Ok(app.set_runner(|mut app: App| 'running: loop {
            if app.state == AppState::Quiting {
                break 'running;
            }

            if app.world.update() {
                app.state = AppState::Quiting
            }
        }))
    }
}
