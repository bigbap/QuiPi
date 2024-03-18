extern crate nalgebra_glm as glm;
extern crate quipi;

use quipi::{
    assets::{AssetId, AssetServer, Assets, Source},
    common::components::components::{CColor, CTexture, CTransform2D},
    ecs::prelude::StorageId::*,
    gfx::{
        prelude::{
            camera_bundle,
            quad::{DefaultQuadShader, QUAD_SHADER_NAME},
            sprite_bundle, CameraMetadata, SpriteMetadata,
        },
        render::{
            assets::{Quad, Texture, TextureLoader},
            renderers::quads::RenderQuads,
            viewport::Viewport,
            RenderBasePlugin,
        },
    },
    prelude::*,
};
use sdl2::keyboard::Keycode;

fn main() {
    if let Err(e) = App::new()
        .add_plugins(default_plugins("Sandbox", 1600, 900))
        .add_plugins(RenderBasePlugin)
        .add_plugins(DefaultQuadShader)
        .add_plugins(MyPlugin)
        .run()
    {
        eprintln!("App shut down unexpectedly: {}", e)
    }
}

struct MyPlugin;

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) -> Result<(), QPError> {
        app.add_system(Startup, setup);
        app.add_system(Update, update);

        let viewport = app.world.resource::<Viewport>().unwrap();
        let viewport = viewport.get_dimensions();
        let interner = app.world.interner_mut();
        let shader_id = AssetId::Id(interner.intern(QUAD_SHADER_NAME));
        let camera_id = app
            .world
            .storage_manager_mut()
            .get_mut(Cameras)
            .unwrap()
            .spawn(camera_bundle(
                MyCamera,
                CameraMetadata {
                    width: viewport.width as u32,
                    height: viewport.height as u32,
                    ..Default::default()
                },
            ));

        app.add_plugins(RenderQuads::new(shader_id, camera_id));

        Ok(())
    }
}

#[derive(Debug, Component, PartialEq, Clone, Copy)]
struct MyCamera;

fn setup(
    asset_server: ResMut<AssetServer>,
    interner: ResMut<StringInterner>,
    storage_manager: ResMut<StorageManager>,
    textures: ResMut<Assets<Texture>>,
) {
    let asset_server = asset_server.unwrap();
    let interner = interner.unwrap();
    let storage_manager = storage_manager.unwrap();
    let textures = textures.unwrap();

    let path = "assets/textures/Bubble.png";
    let texture = asset_server
        .load(TextureLoader {
            source: Source::Path(path),
            dims: None,
        })
        .unwrap();

    // asset_server.load(Quad).unwrap();

    let id = interner.intern(path);
    let texture_handle = textures.add(id, texture);

    storage_manager
        .get_mut(Entities)
        .unwrap()
        .spawn(sprite_bundle(SpriteMetadata {
            texture: Some(CTexture {
                handle: texture_handle,
                atlas_location: None,
            }),
            transform: CTransform2D {
                translate: glm::vec2(200.0, 100.0),
                ..CTransform2D::default()
            },
            color: Some(CColor(1.0, 0.1, 0.2, 1.0)),
            ..SpriteMetadata::default()
        }));
}

fn update(clock: ResMut<Clock>, input: Res<Input>) {
    let clock = clock.unwrap();
    let elapsed = clock.elapsed();

    let input = input.unwrap();
    if let Some(_) = input.peek(Keycode::W) {
        println!("pressing W at {}", elapsed);
    }
}
