use qp_common::components::*;
use qp_core::{trig::pi, Countdown};
use quipi::{
    assets::AssetHandle,
    core::math::random::Random,
    gfx::{
        prelude::{sprite_bundle, SpriteMetadata},
        render::{assets::Texture, viewport::Viewport},
    },
    prelude::*,
    resources::{AsAny, Resource},
    QPResult,
};
use sdl2::keyboard::Keycode;

use crate::{camera::CameraState, GameState};

const ACCELERATION: f32 = 100.0;
const MAX_VELOCITY: f32 = 500.0;
const EXHAUST_INTERVAL: u128 = 5;
const EXHAUST_PARTICLE_LIFETIME: u128 = 250;
const BULLET_SPAWN_INTERVAL: u128 = 200;
const BULLET_LIFETIME: u128 = 1300;
const BULLET_SPEED: f32 = 700.0;

pub struct Ship;
impl Plugin for Ship {
    fn build(&self, app: &mut App) -> QPResult<()> {
        app.add_system(Startup, setup)
            .add_system(Update, handle_input)
            .add_system(Update, fly)
            .add_plugins(Bullets)
            .add_plugins(ExhaustSystem);

        Ok(())
    }
}

#[derive(Resource, AsAny)]
pub struct PlayerState {
    pub id: Index,
    pub thrust: bool,
    pub firing: bool,
}

fn setup(
    world: &mut World,
    storage_manager: ResMut<StorageManager>,
    game_state: Res<GameState>,
    camera_state: ResMut<CameraState>,
) {
    let storage_manager = storage_manager.unwrap();
    let game_state = game_state.unwrap();
    let camera_state = camera_state.unwrap();
    let texture_handle = game_state
        .texture_handle
        .as_ref()
        .expect("texture hasn't been loaded")
        .clone();

    let ship = storage_manager
        .get_mut(StorageId::Entities)
        .unwrap()
        .spawn((
            CVelocity2D { x: 0.0, y: 0.0 },
            sprite_bundle(SpriteMetadata {
                texture: Some(CTexture {
                    handle: texture_handle,
                    atlas_location: Some((1, 3)),
                }),
                transform: CTransform2D {
                    translate: glm::vec2(200.0, 100.0),
                    ..CTransform2D::default()
                },
                color: Some(CColor(1.0, 0.1, 0.2, 1.0)),
                quad: CQuad {
                    width: 64.0,
                    height: 64.0,
                    center_x: 0.0,
                    center_y: 0.0,
                },
                ..SpriteMetadata::default()
            }),
        ));

    camera_state.follow = Some(ship);

    world
        .resources
        .insert(PlayerState {
            id: ship,
            thrust: false,
            firing: false,
        })
        .expect("failed to add ship resource");
}

fn handle_input(
    state: ResMut<PlayerState>,
    input: Res<Input>,
    viewport: Res<Viewport>,
    storage: ResMut<StorageManager>,
) {
    let (Some(state), Some(viewport), Some(input), Some(storage)) =
        (state, viewport, input, storage)
    else {
        return;
    };

    if input.mouse_moved() {
        rotate(state, input, viewport, storage);
    }

    state.thrust = input.peek(Keycode::W).is_some();
    state.firing = input.peek(Keycode::Space).is_some();
}

fn rotate(state: &PlayerState, input: &Input, viewport: &Viewport, storage: &mut StorageManager) {
    let mouse_pos = input.mouse_position();
    let dims = viewport.get_dimensions();
    let x = mouse_pos.x as f32 - dims.width as f32 / 2.0;
    let y = (mouse_pos.y as f32 - dims.height as f32 / 2.0) * -1.0;

    let angle =
        qp_core::trig::angle(&glm::vec3(0.0, 1.0, 0.0), &glm::vec3(x, y, 0.0)) - (pi() / 2.0);

    if let Some(transform) = storage
        .get_mut(StorageId::Entities)
        .unwrap()
        .get_mut::<CTransform2D>(&state.id)
    {
        transform.rotate = angle;
    }
}

fn fly(state: ResMut<PlayerState>, game_state: Res<GameState>, storage: ResMut<StorageManager>) {
    let (Some(state), Some(game_state), Some(storage)) = (state, game_state, storage) else {
        return;
    };

    let Some(velocity) = storage
        .get_mut(StorageId::Entities)
        .unwrap()
        .get_mut::<CVelocity2D>(&state.id)
    else {
        return;
    };

    if !state.thrust && velocity.x <= 0.0 && velocity.y <= 0.0 {
        return;
    }

    let acceleration = game_state.delta
        * match state.thrust {
            true => ACCELERATION,
            false => -ACCELERATION,
        };

    velocity.x = (velocity.x + acceleration).clamp(0.0, MAX_VELOCITY);
    velocity.y = (velocity.y + acceleration).clamp(0.0, MAX_VELOCITY);

    // apply velocity to translation
    let velocity = velocity.clone();
    if let Some(transform) = storage
        .get_mut(StorageId::Entities)
        .unwrap()
        .get_mut::<CTransform2D>(&state.id)
    {
        let direction = transform.direction();
        transform.translate.x += velocity.x * game_state.delta * direction.x;
        transform.translate.y += velocity.y * game_state.delta * direction.y;
    }
}

struct ExhaustSystem;
impl Plugin for ExhaustSystem {
    fn build(&self, app: &mut App) -> QPResult<()> {
        let mut interval = Interval::new(EXHAUST_INTERVAL);

        app.add_system(
            Update,
            move |player_state: Res<PlayerState>,
                  storage: ResMut<StorageManager>,
                  rand: ResMut<Random>| {
                let (Some(state), Some(storage), Some(rand)) = (player_state, storage, rand) else {
                    return;
                };

                if state.thrust && interval.check() {
                    let Some(transform) = storage
                        .get(StorageId::Entities)
                        .unwrap()
                        .get::<CTransform2D>(&state.id)
                    else {
                        return;
                    };

                    let offset = transform.direction() * 28.0;
                    let scale_factor = rand.random() * 0.6;
                    let translate = transform.translate - offset;
                    storage.get_mut(StorageId::Entities).unwrap().spawn((
                        CParticle {
                            countdown: Countdown::new(EXHAUST_PARTICLE_LIFETIME),
                        },
                        sprite_bundle(SpriteMetadata {
                            transform: CTransform2D {
                                translate,
                                rotate: rand.random() * 2.0 * glm::pi::<f32>(),
                                scale: glm::vec2(scale_factor, scale_factor),
                                ..CTransform2D::default()
                            },
                            color: Some(CColor(1.0, 0.7, 0.2, 1.0)),
                            quad: CQuad {
                                width: 32.0,
                                height: 32.0,
                                center_x: 0.0,
                                center_y: 0.0,
                            },
                            ..SpriteMetadata::default()
                        }),
                    ));
                }
            },
        );

        Ok(())
    }
}

pub struct Bullets;
impl Plugin for Bullets {
    fn build(&self, app: &mut App) -> QPResult<()> {
        app.add_system(Startup, setup_bullets)
            .add_system(Update, spawn_bullet)
            .add_system(Update, update_bullets);

        Ok(())
    }
}

#[derive(Debug, Resource, AsAny)]
struct BulletsState {
    pub texture_handle: AssetHandle<Texture>,
    pub spawn_interval: Interval,
}

#[derive(Debug, Component, PartialEq, Clone)]
pub struct CBullet;

fn setup_bullets(world: &mut World, game_state: Res<GameState>) {
    let texture_handle = game_state
        .unwrap()
        .texture_handle
        .as_ref()
        .expect("texture hasn't been loaded")
        .clone();

    world
        .resources
        .insert(BulletsState {
            texture_handle,
            spawn_interval: Interval::new(BULLET_SPAWN_INTERVAL),
        })
        .expect("failed to add bullets resource");
}

fn spawn_bullet(
    state: ResMut<BulletsState>,
    storage_manager: ResMut<StorageManager>,
    player_state: Res<PlayerState>,
) {
    let (Some(state), Some(storage), Some(player_state)) = (state, storage_manager, player_state)
    else {
        return;
    };

    if !state.spawn_interval.check() || !player_state.firing {
        return;
    }

    let Some(entities) = storage.get_mut(StorageId::Entities) else {
        return;
    };

    let Some(ship) = entities.get::<CTransform2D>(&player_state.id) else {
        return;
    };

    let direction = ship.direction();
    let position = ship.translate + (direction * 28.0);

    single_spawn(entities, state, &direction, position, ship.rotate);
}

fn single_spawn(
    storage: &mut Storage,
    state: &BulletsState,
    direction: &glm::Vec2,
    position: glm::Vec2,
    angle: f32,
) {
    storage.spawn((
        CBullet,
        CParticle {
            countdown: Countdown::new(BULLET_LIFETIME),
        },
        CVelocity2D {
            x: BULLET_SPEED * direction.x,
            y: BULLET_SPEED * direction.y,
        },
        sprite_bundle(SpriteMetadata {
            texture: Some(CTexture {
                handle: state.texture_handle.clone(),
                atlas_location: Some((1, 5)),
            }),
            transform: CTransform2D {
                translate: position,
                rotate: angle,
                scale: glm::vec2(0.5, 0.5),
                ..CTransform2D::default()
            },
            color: Some(CColor(0.0, 0.7, 1.0, 1.0)),
            quad: CQuad {
                width: 64.0,
                height: 64.0,
                ..CQuad::default()
            },
            ..SpriteMetadata::default()
        }),
    ));
}

fn update_bullets(storage_manager: ResMut<StorageManager>, game_state: Res<GameState>) {
    let (Some(storage), Some(game_state)) = (storage_manager, game_state) else {
        return;
    };

    let Some(entities) = storage.get_mut(StorageId::Entities) else {
        return;
    };

    let iterator = match entities.iter_mut::<CBullet>() {
        Some(p) => p,
        _ => return,
    };

    let mut to_change: Vec<Index> = vec![];
    for bullet in iterator {
        if bullet.is_none() {
            continue;
        }

        let (entity, _) = bullet.unwrap();

        to_change.push(entity);
    }

    for entity in to_change.iter() {
        let Some(velocity) = entities.get::<CVelocity2D>(&entity) else {
            continue;
        };

        let velocity = glm::vec2(velocity.x * game_state.delta, velocity.y * game_state.delta);

        if let Some(transform) = entities.get_mut::<CTransform2D>(&entity) {
            transform.translate += velocity
        } else {
            continue;
        }
    }
}
