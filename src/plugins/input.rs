use sdl2::event::Event;

use crate::{
    registry::GlobalRegistry,
    resource_manager::resources::{input::Input, window::Window},
    QPResult,
};

use super::Plugin;

pub struct InputPlugin {}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        let window_id = app
            .world
            .registry
            .resources
            .resource_id::<Window>()
            .unwrap();

        let input_id = app.world.registry.resources.resource_id::<Input>().unwrap();

        app.add_resource(Input::new());

        app.add_system(move |registry: &mut GlobalRegistry| {
            let Some(window) = registry.resources.get::<Window>(window_id) else {
                #[cfg(debug_assertions)]
                println!("[input system] couldn't get window resource");

                return;
            };

            let events = window.winapi.get_event_queue().unwrap();

            let Some(input) = registry.resources.get_mut::<Input>(input_id) else {
                #[cfg(debug_assertions)]
                println!("[input system] couldn't get input resource");

                return;
            };

            for event in events.iter() {
                match event {
                    Event::Quit { .. } => registry.quit = true,
                    event => input.update_state(event),
                }
            }
        });

        Ok(())
    }
}
