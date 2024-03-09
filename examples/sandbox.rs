use quipi::{
    app::App,
    errors::QPError,
    plugins::{default_plugins, plugins_2d, Plugin},
    registry::GlobalRegistry,
    resource_manager::resources::{clock::Clock, input::Input},
};
use sdl2::keyboard::Keycode;

fn main() {
    if let Err(e) = App::new()
        .add_plugins(default_plugins())
        .add_plugins(plugins_2d())
        .add_plugins(MyPlugin {})
        .run()
    {
        eprintln!("App shut down unexpectedly: {}", e)
    }
}

struct MyPlugin {}

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) -> Result<(), QPError> {
        let clock_id = app.world.registry.resources.resource_id::<Clock>().unwrap();
        let input_id = app.world.registry.resources.resource_id::<Input>().unwrap();

        app.add_system(move |registry: &mut GlobalRegistry| {
            let clock = registry.resources.get_mut::<Clock>(clock_id).unwrap();
            let elapsed = clock.elapsed();

            let input = registry.resources.get::<Input>(input_id).unwrap();
            if let Some(_) = input.peek(Keycode::W) {
                println!("pressing W at {}", elapsed);
            }
        });

        Ok(())
    }
}
