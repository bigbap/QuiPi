extern crate nalgebra_glm as glm;
extern crate quipi;

use quipi::{
    common::{
        assets::{ShaderAsset, TextureAsset, TextureLoader},
        bundles::{sprite_bundle, SpriteMetadata},
        components::components::{CColor, CTexture, CTransform2D},
        plugins::quad_shader::QUAD_SHADER_NAME,
        systems::render_quads,
    },
    prelude::*,
};
use sdl2::keyboard::Keycode;

fn main() {
    if let Err(e) = App::new()
        .add_plugins(default_plugins("Sandbox", 1600, 900))
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
        app.add_system::<StartupSchedule>(|world: &mut World| {
            world.load_asset(
                "bubble_texture".into(),
                TextureLoader {
                    source: Source::Path("assets/textures/Bubble.png"),
                    dims: None,
                },
            )?;

            let texture_id = world.resources.asset_id::<TextureAsset>("bubble_texture")?;

            world.spawn(sprite_bundle(SpriteMetadata {
                texture: Some(CTexture {
                    id: texture_id,
                    atlas_location: None,
                }),
                transform: CTransform2D {
                    translate: glm::vec2(200.0, 100.0),
                    ..CTransform2D::default()
                },
                color: Some(CColor(1.0, 0.1, 0.2, 1.0)),
                ..SpriteMetadata::default()
            }))?;

            Ok(())
        });

        app.add_system::<UpdateSchedule>(move |world: &mut World| {
            let clock = world
                .resources
                .get_mut::<Clock>()
                .ok_or(QPError::ResourceNotFound("Clock".to_string()))?;
            let elapsed = clock.elapsed();

            let input = world
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
            .resources
            .asset_id::<ShaderAsset>(QUAD_SHADER_NAME)?;
        let camera_id = app
            .world
            .add_camera("my_camera".into(), Camera2D::default())?;

        app.add_system::<RenderSchedule>(render_quads(shader_id, camera_id));

        Ok(())
    }
}
