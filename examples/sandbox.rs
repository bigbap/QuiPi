use quipi::{
    common::{
        assets::{ShaderAsset, TextureAsset, TextureLoader},
        bundles::sprite_bundle,
        components::components::{CColor, CQuad},
        plugins::quad_shader::QUAD_SHADER_NAME,
        systems::render_quads,
    },
    prelude::*,
};
use sdl2::keyboard::Keycode;

fn main() {
    if let Err(e) = App::new()
        .add_plugins(window_plugins("Sandbox", 1600, 900))
        .add_plugins(render_plugins())
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
            CColor(1.0, 0.1, 0.2, 1.0),
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

        let shader_id = app
            .world
            .registry
            .resources
            .get_asset_id::<ShaderAsset>(QUAD_SHADER_NAME)?;
        let camera_id = app
            .world
            .registry
            .resources
            .add_camera("my_camera", Camera2D::default())?;

        app.add_system::<RenderSchedule>(render_quads(shader_id, camera_id));

        Ok(())
    }
}
