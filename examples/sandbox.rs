extern crate nalgebra_glm as glm;
extern crate quipi;

use quipi::{
    assets::{AssetServer, Assets, Source},
    common::components::components::{CColor, CTexture, CTransform2D},
    ecs::prelude::StorageId::*,
    gfx::{
        prelude::{camera_bundle, sprite_bundle, CameraMetadata, SpriteMetadata},
        render::{
            assets::{Texture, TextureLoader},
            RenderBasePlugin,
        },
    },
    prelude::*,
};
use sdl2::keyboard::Keycode;

fn main() {
    if let Err(e) = App::new()
        .add_plugins(default_plugins("Sandbox", 1600, 900))
        .add_plugins(RenderBasePlugin::default())
        .add_plugins(MyPlugin {})
        .run()
    {
        eprintln!("App shut down unexpectedly: {}", e)
    }
}

struct MyPlugin {}

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) -> Result<(), QPError> {
        app.add_system::<StartupSchedule, _, _>(|| Ok(()));
        // app.add_system::<StartupSchedule>(|world: &mut World| {
        //     let path = "assets/textures/Bubble.png";
        //     let texture = world
        //         .resource_mut::<AssetServer>()
        //         .unwrap()
        //         .load(TextureLoader {
        //             source: Source::Path(path),
        //             dims: None,
        //         })?;

        //     let id = world.interner_mut().intern(path);
        //     let texture_handle = world
        //         .resource_mut::<Assets<Texture>>()
        //         .unwrap()
        //         .add(id, texture);

        //     world
        //         .storage_mut()
        //         .get_mut(Entities)
        //         .ok_or(QPError::Generic("entities not found".into()))?
        //         .spawn(sprite_bundle(SpriteMetadata {
        //             texture: Some(CTexture {
        //                 id: texture_handle,
        //                 atlas_location: None,
        //             }),
        //             transform: CTransform2D {
        //                 translate: glm::vec2(200.0, 100.0),
        //                 ..CTransform2D::default()
        //             },
        //             color: Some(CColor(1.0, 0.1, 0.2, 1.0)),
        //             ..SpriteMetadata::default()
        //         }));

        //     world
        //         .storage_mut()
        //         .get_mut(Entities)
        //         .ok_or(QPError::Generic("entities not found".into()))?
        //         .spawn(camera_bundle(CameraMetadata::default()));

        //     Ok(())
        // });

        // app.add_system::<UpdateSchedule>(move |world: &mut World| {
        //     let clock = world
        //         .resources
        //         .get_mut::<Clock>()
        //         .ok_or(QPError::ResourceNotFound("Clock".to_string()))?;
        //     let elapsed = clock.elapsed();

        //     let input = world
        //         .resources
        //         .get::<Input>()
        //         .ok_or(QPError::ResourceNotFound("Input".to_string()))?;
        //     if let Some(_) = input.peek(Keycode::W) {
        //         println!("pressing W at {}", elapsed);
        //     }

        //     Ok(())
        // });

        // let shader_id = app.world.resources.asset_id::<Shader>(QUAD_SHADER_NAME)?;
        // let camera_id = app
        //     .world
        //     .add_camera("my_camera".into(), Camera2D::default())?;

        // app.add_system::<RenderSchedule>(render_quads(shader_id, camera_id));

        Ok(())
    }
}
