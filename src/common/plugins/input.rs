use sdl2::event::Event;

use crate::{
    common::resources::{input::Input, window::Window},
    prelude::{Res, ResMut, World},
    schedule::Update,
    QPResult,
};

use super::Plugin;

pub struct InputPlugin {}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        app.add_resource(Input::new());

        app.add_system(
            Update,
            move |world: &mut World, window: Res<Window>, input: ResMut<Input>| {
                let Some(window) = window else {
                    #[cfg(debug_assertions)]
                    println!("[input system] couldn't get window resource");

                    return;
                };

                let events = window.winapi.get_event_queue().unwrap();
                let Some(input) = input else {
                    #[cfg(debug_assertions)]
                    println!("[input system] couldn't get input resource");

                    return;
                };

                let mut quit = false;
                for event in events.iter() {
                    match event {
                        Event::Quit { .. } => quit = true,
                        event => input.update_state(event),
                    }
                }

                if quit {
                    world.quit();
                }
            },
        );

        Ok(())
    }
}
