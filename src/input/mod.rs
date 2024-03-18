use sdl2::event::{Event, WindowEvent};

use crate::{
    gfx::render::viewport::ViewportDimensions,
    plugin::Plugin,
    prelude::{
        qp_gfx::{Viewport, Window},
        Res, ResMut, World,
    },
    schedule::Update,
    QPResult,
};

pub mod state;
pub use state::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        app.add_resource(Input::new());

        app.add_system(
            Update,
            move |world: &mut World,
                  window: Res<Window>,
                  input: ResMut<Input>,
                  mut viewport: ResMut<Viewport>| {
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
                        Event::Window {
                            win_event: WindowEvent::Resized(w, h),
                            ..
                        } => match &mut viewport {
                            Some(v) => v.set_dimensions(ViewportDimensions {
                                x: 0,
                                y: 0,
                                width: *w,
                                height: *h,
                            }),
                            _ => (),
                        },
                        event => input.update_state(&event),
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
