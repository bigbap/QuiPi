use sdl2::event::Event;

use crate::{
    common::resources::{input::Input, window::Window},
    prelude::{QPError, World},
    resources::ResourceManager,
    world::UpdateSchedule,
    QPResult,
};

use super::Plugin;

pub struct InputPlugin {}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        app.add_resource(Input::new());

        app.add_system::<UpdateSchedule>(move |resources: &mut ResourceManager| {
            let Some(window) = resources.get::<Window>() else {
                #[cfg(debug_assertions)]
                println!("[input system] couldn't get window resource");

                return Ok(());
            };

            let events = window.winapi.get_event_queue().unwrap();

            let Some(input) = resources.get_mut::<Input>() else {
                #[cfg(debug_assertions)]
                println!("[input system] couldn't get input resource");

                return Ok(());
            };

            for event in events.iter() {
                match event {
                    Event::Quit { .. } => return Err(QPError::Quit),
                    event => input.update_state(event),
                }
            }

            Ok(())
        });

        Ok(())
    }
}
