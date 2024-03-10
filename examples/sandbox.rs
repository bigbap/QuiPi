use quipi::{
    common::{
        assets::{TextureAsset, TextureLoader},
        bundles::sprite_bundle,
        components::components::{CColor, CQuad},
    },
    prelude::*,
};
use sdl2::keyboard::Keycode;

fn main() {
    if let Err(e) = App::new()
        .add_plugins(plugins_2d("Sandbox", 1600, 900))
        .add_plugins(MyPlugin {})
        .run()
    {
        eprintln!("App shut down unexpectedly: {}", e)
    }
}

struct MyPlugin {}

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) -> Result<(), QPError> {
        app.load_asset(
            "bubble_texture",
            TextureLoader {
                source: Source::Path("assets/textures/Bubble.png"),
                dims: None,
            },
        );

        let texture_id = app
            .world
            .registry
            .resources
            .get_asset_id::<TextureAsset>("bubble_texture")?;

        app.world.registry.entities.create(sprite_bundle(
            CQuad::default(),
            texture_id,
            (1, 1),
            CColor(0.7, 0.1, 0.2, 1.0),
        ));

        app.add_system::<UpdateSchedule>(move |registry: &mut GlobalRegistry| {
            let clock = registry
                .resources
                .get_mut::<Clock>()
                .ok_or(QPError::ResourceNotFound("Clock".to_string()))?;
            let elapsed = clock.elapsed();

            let input = registry
                .resources
                .get::<Input>()
                .ok_or(QPError::ResourceNotFound("Input".to_string()))?;
            if let Some(_) = input.peek(Keycode::W) {
                println!("pressing W at {}", elapsed);
            }

            Ok(())
        });

        Ok(())
    }
}
