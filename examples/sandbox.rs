extern crate nalgebra_glm as glm;
extern crate quipi;

use std::marker::PhantomData;

use quipi::{
    assets::{AssetId, AssetServer, Assets, Source},
    common::components::components::{CColor, CTexture, CTransform2D},
    ecs::prelude::StorageId::*,
    gfx::{
        prelude::{
            camera_bundle, quad::QUAD_SHADER_NAME, sprite_bundle, CameraMetadata, SpriteMetadata,
        },
        render::{
            assets::{Quad, Shader, Texture, TextureLoader},
            RenderBasePlugin,
        },
    },
    prelude::*,
    QPResult,
};
use sdl2::keyboard::Keycode;

fn main() {
    if let Err(e) = App::new()
        .add_plugins(default_plugins("Sandbox", 1600, 900))
        .add_plugins(RenderBasePlugin::default())
        .add_system(Startup, |world: &mut World| {
            world.asset_server().load(Quad)?;
            Ok(())
        })
        .add_plugins(MyPlugin {})
        .run()
    {
        eprintln!("App shut down unexpectedly: {}", e)
    }
}

struct MyPlugin {}

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) -> Result<(), QPError> {
        app.add_system(Startup, |world: &mut World| {
            let path = "assets/textures/Bubble.png";
            let texture = world.asset_server().load(TextureLoader {
                source: Source::Path(path),
                dims: None,
            })?;

            let id = world.interner_mut().intern(path);
            let texture_handle = world.assets_mut::<Texture>().unwrap().add(id, texture);

            world
                .storage_manager_mut()
                .get_mut(Entities)
                .unwrap()
                .spawn(sprite_bundle(SpriteMetadata {
                    texture: Some(CTexture {
                        id: texture_handle,
                        atlas_location: None,
                    }),
                    transform: CTransform2D {
                        translate: glm::vec2(200.0, 100.0),
                        ..CTransform2D::default()
                    },
                    color: Some(CColor(1.0, 0.1, 0.2, 1.0)),
                    ..SpriteMetadata::default()
                }));

            Ok(())
        });

        app.add_system(Update, move |world: &mut World| {
            let clock = world
                .resource_mut::<Clock>()
                .ok_or(QPError::ResourceNotFound("Clock".to_string()))?;
            let elapsed = clock.elapsed();

            let input = world
                .resource::<Input>()
                .ok_or(QPError::ResourceNotFound("Input".to_string()))?;
            if let Some(_) = input.peek(Keycode::W) {
                println!("pressing W at {}", elapsed);
            }

            Ok(())
        });

        let interner = app.world.interner_mut();
        let interned = interner.intern(QUAD_SHADER_NAME);
        // let shader_id = app
        //     .world
        //     .assets::<Shader>()
        //     .unwrap()
        //     .get(&AssetId::Id(interned))
        //     .unwrap();
        let camera_id = app
            .world
            .storage_manager_mut()
            .get_mut(Cameras)
            .unwrap()
            .spawn(camera_bundle(CameraMetadata::default()));

        // app.add_system(Render, render_quads(shader_id, camera_id));

        Ok(())
    }
}
