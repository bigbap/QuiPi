use quipi::{
    assets::AssetHandle,
    common::components::components::{CColor, CQuad, CTexture, CTransform2D, CVelocity2D},
    core::{
        math::{
            random::Random,
            trig::{magnitude2d_squared, rotate2d},
        },
        time::{Countdown, Interval},
    },
    gfx::{
        prelude::{sprite_bundle, SpriteMetadata},
        render::{assets::Texture, viewport::Viewport},
    },
    particle_system::CParticle,
    plugin::Plugin,
    prelude::{Component, Index, Res, ResMut, StorageId, StorageManager, World},
    resources::{AsAny, Resource},
    schedule::{Startup, Update},
    QPResult,
};

use crate::{
    ship::{CBullet, PlayerState},
    GameState,
};

const ASTEROID_INTERVAL: u128 = 300;
const ASTEROIDS_TO_SPAWN: u32 = 1;
const ASTEROID_LIFETIME: u128 = 5000;

pub struct Asteroids;

impl Plugin for Asteroids {
    fn build(&self, app: &mut quipi::prelude::App) -> QPResult<()> {
        app.add_system(Startup, setup)
            .add_system(Update, spawn)
            .add_system(Update, update);

        Ok(())
    }
}

#[derive(Resource, AsAny)]
struct AsteroidsState {
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
        .insert(AsteroidsState {
            interval: Interval::new(ASTEROID_INTERVAL),
            texture_handle,
        })
        .expect("failed to add stars resource");
}

fn spawn(
    state: ResMut<AsteroidsState>,
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
    let offset = 100.0;
    let dims = viewport.get_dimensions();
    let pos_local = glm::vec2(
        (dims.width / 2) as f32 + offset,
        (dims.height / 2) as f32 + offset,
    );
    let pos_local = rotate2d(&pos_local, rand.random() * 2.0 * glm::pi::<f32>());
    let pos = ship_pos + pos_local;

    for _ in 0..ASTEROIDS_TO_SPAWN {
        single_spawn(
            &state,
            &ship_pos,
            pos,
            (rand.random() + 1.0) * 2.0,
            rand.random() * 2.0 * glm::pi::<f32>(),
            rand,
            storage,
        );
    }
}

fn single_spawn(
    state: &AsteroidsState,
    ship_pos: &glm::Vec2,
    position: glm::Vec2,
    scale: f32,
    rotate: f32,
    rand: &mut Random,
    storage: &mut StorageManager,
) {
    let direction = (ship_pos - position).normalize();

    storage.get_mut(StorageId::Entities).unwrap().spawn((
        CAsteroid {
            rotation_step: (rand.random() + 0.7) * 2.0,
        },
        CParticle {
            countdown: Countdown::new(ASTEROID_LIFETIME),
        },
        CVelocity2D {
            x: direction.x * rand.range(150, 300) as f32,
            y: direction.y * rand.range(150, 300) as f32,
        },
        sprite_bundle(SpriteMetadata {
            texture: Some(CTexture {
                handle: state.texture_handle.clone(),
                atlas_location: Some((0, 1)),
            }),
            transform: CTransform2D {
                translate: position,
                scale: glm::vec2(scale, scale),
                rotate,
                ..CTransform2D::default()
            },
            color: Some(CColor(0.9, 0.9, 0.9, 1.0)),
            quad: CQuad {
                width: 32.0,
                height: 32.0,
                ..CQuad::default()
            },
            ..SpriteMetadata::default()
        }),
    ));
}

fn update(
    storage: ResMut<StorageManager>,
    game_state: ResMut<GameState>,
    player_state: Res<PlayerState>,
) {
    let (Some(storage), Some(game_state), Some(ship)) = (storage, game_state, player_state) else {
        return;
    };

    let iterator = match storage
        .get(StorageId::Entities)
        .unwrap()
        .iter::<CVelocity2D>()
    {
        Some(p) => p,
        _ => return,
    };

    let bullets: Vec<(Index, &CBullet)> =
        match storage.get(StorageId::Entities).unwrap().iter::<CBullet>() {
            Some(iter) => iter.filter_map(|b| b).collect(),
            _ => vec![],
        };

    let asteroids: Vec<(Index, &CVelocity2D)> = iterator.filter_map(|b| b).collect();

    let mut to_delete: Vec<Index> = vec![];
    let mut to_change: Vec<(Index, f32, f32, f32)> = vec![];
    for asteroid in asteroids {
        let (asteroid, velocity) = asteroid;

        if storage
            .get(StorageId::Entities)
            .unwrap()
            .get::<CAsteroid>(&asteroid)
            .is_none()
        {
            continue;
        }

        let Some(asteroid_transform) = storage
            .get(StorageId::Entities)
            .unwrap()
            .get::<CTransform2D>(&asteroid)
        else {
            continue;
        };

        let Some(ship_transform) = storage
            .get(StorageId::Entities)
            .unwrap()
            .get::<CTransform2D>(&ship.id)
        else {
            continue;
        };

        if check_collision(asteroid_transform, ship_transform, 16.0) {
            to_delete.push(ship.id.clone());
            to_delete.push(asteroid.clone());

            game_state.game_over = true;

            continue;
        }

        let mut deleting = false;
        for (bullet, _) in bullets.iter() {
            let Some(bullet_transform) = storage
                .get(StorageId::Entities)
                .unwrap()
                .get::<CTransform2D>(&bullet)
            else {
                continue;
            };

            if check_collision(asteroid_transform, bullet_transform, 16.0) {
                to_delete.push(bullet.clone());
                to_delete.push(asteroid.clone());
                deleting = true;

                break;
            }
        }

        if deleting {
            continue;
        }

        let Some(asteroid_cmp) = storage
            .get(StorageId::Entities)
            .unwrap()
            .get::<CAsteroid>(&asteroid)
        else {
            continue;
        };

        to_change.push((asteroid, velocity.x, velocity.y, asteroid_cmp.rotation_step));
    }

    for entity in to_delete {
        storage
            .get_mut(StorageId::Entities)
            .unwrap()
            .despwan(entity);
    }

    for (entity, x, y, rotation_step) in to_change.iter() {
        let Some(transform) = storage
            .get_mut(StorageId::Entities)
            .unwrap()
            .get_mut::<CTransform2D>(&entity)
        else {
            continue;
        };

        transform.translate.x += x * game_state.delta;
        transform.translate.y += y * game_state.delta;

        transform.rotate += game_state.delta * rotation_step;
    }
}

fn check_collision(asteroid: &CTransform2D, obj: &CTransform2D, obj_radius: f32) -> bool {
    let offset = 25.0;
    let threshold = obj_radius + (asteroid.scale.x * 16.0) - offset;
    magnitude2d_squared(&asteroid.translate, &obj.translate) < threshold.powf(2.0)
}

#[derive(Debug, Component, PartialEq, Clone)]
struct CAsteroid {
    rotation_step: f32,
}
