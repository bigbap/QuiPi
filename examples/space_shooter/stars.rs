use quipi::{
    assets::AssetHandle,
    common::components::components::{CColor, CQuad, CTexture, CTransform2D},
    core::{
        math::random::Random,
        time::{Countdown, Interval},
    },
    gfx::{
        prelude::{sprite_bundle, SpriteMetadata},
        render::{
            assets::Texture,
            viewport::{Viewport, ViewportDimensions},
        },
    },
    particle_system::CParticle,
    plugin::Plugin,
    prelude::{Res, ResMut, StorageId, StorageManager, World},
    resources::{AsAny, Resource},
    schedule::{Startup, Update},
    QPResult,
};

use crate::{ship::PlayerState, GameState};

const STAR_INTERVAL: u128 = 1;
const PARTICLE_LIFETIME: u128 = 1000;

pub struct Stars;

impl Plugin for Stars {
    fn build(&self, app: &mut quipi::prelude::App) -> QPResult<()> {
        app.add_system(Startup, setup);
        app.add_system(Update, spawn);

        Ok(())
    }
}

#[derive(Resource, AsAny)]
struct StarsState {
    pub interval: Interval,
    pub texture_handle: AssetHandle<Texture>,
}

fn setup(world: &mut World, game_state: Res<GameState>) {
    let texture_handle = game_state
        .unwrap()
        .texture_handle
        .as_ref()
        .expect("texture hasn't been loaded")
        .clone();

    world
        .resources
        .insert(StarsState {
            interval: Interval::new(STAR_INTERVAL),
            texture_handle,
        })
        .expect("failed to add stars resource");
}

fn spawn(
    state: ResMut<StarsState>,
    ship_state: Res<PlayerState>,
    storage_manager: ResMut<StorageManager>,
    viewport: Res<Viewport>,
    rand: ResMut<Random>,
) {
    let (Some(state), Some(ship_state), Some(storage), Some(rand), Some(viewport)) =
        (state, ship_state, storage_manager, rand, viewport)
    else {
        return;
    };

    // let count = storage.get(StorageId::Entities).unwrap().len();
    // println!("valid entity count: {}", count);

    if !state.interval.check() {
        return;
    }

    let Some(transform) = storage
        .get(StorageId::Entities)
        .unwrap()
        .get::<CTransform2D>(&ship_state.id)
    else {
        return;
    };
    let ship_pos = transform.translate;

    let dims = viewport.get_dimensions();

    for _ in 0..10 {
        single_spawn(&state, &ship_pos, rand, &dims, storage);
    }
}

fn single_spawn(
    state: &StarsState,
    ship_pos: &glm::Vec2,
    rand: &mut Random,
    dims: &ViewportDimensions,
    storage: &mut StorageManager,
) {
    let x_pos = rand.range(
        ship_pos.x as i32 - dims.width,
        ship_pos.x as i32 + dims.width,
    ) as f32;
    let y_pos = rand.range(
        ship_pos.y as i32 - dims.height,
        ship_pos.y as i32 + dims.height,
    ) as f32;

    storage.get_mut(StorageId::Entities).unwrap().spawn((
        CParticle {
            countdown: Countdown::new(PARTICLE_LIFETIME),
        },
        sprite_bundle(SpriteMetadata {
            texture: Some(CTexture {
                handle: state.texture_handle.clone(),
                atlas_location: Some((
                    match rand.binary(0.7) {
                        true => 7,
                        false => 6,
                    },
                    2,
                )),
            }),
            transform: CTransform2D {
                translate: glm::vec2(x_pos, y_pos),
                scale: glm::vec2(0.7, 0.7),
                ..CTransform2D::default()
            },
            color: Some(CColor(1.0, 1.0, 0.8, 1.0)),
            quad: CQuad {
                width: 32.0,
                height: 32.0,
                ..CQuad::default()
            },
            ..SpriteMetadata::default()
        }),
    ));
}
