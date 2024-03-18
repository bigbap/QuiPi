use qp_common::components::*;
use qp_core::{trig::pi, Countdown};
use quipi::{
    core::math::random::Random,
    gfx::{
        prelude::{sprite_bundle, SpriteMetadata},
        render::viewport::Viewport,
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
const PARTICLE_LIFETIME: u128 = 250;

pub struct Ship;
impl Plugin for Ship {
    fn build(&self, app: &mut App) -> QPResult<()> {
        app.add_system(Startup, setup)
            .add_system(Update, handle_input)
            .add_system(Update, fly)
            .add_plugins(ExhaustSystem);

        Ok(())
    }
}

#[derive(Resource, AsAny)]
pub struct PlayerState {
    pub id: Index,
    pub thrust: bool,
    pub delta: f32,
    pub timer: Timer,
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
            delta: 0.0,
            timer: Timer::new(),
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

    // ONLY UPDATE THE DELTA HERE
    state.delta = state.timer.delta() as f32 / 1000.0;

    if input.mouse_moved() {
        rotate(state, input, viewport, storage);
    }

    state.thrust = input.peek(Keycode::W).is_some();
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

fn fly(state: ResMut<PlayerState>, storage: ResMut<StorageManager>) {
    let (Some(state), Some(storage)) = (state, storage) else {
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

    let acceleration = state.delta
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
        transform.translate.x += velocity.x * state.delta * direction.x;
        transform.translate.y += velocity.y * state.delta * direction.y;
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
                    let scale_factor = rand.random() * 0.8;
                    let translate = transform.translate - offset;
                    storage.get_mut(StorageId::Entities).unwrap().spawn((
                        CParticle {
                            countdown: Countdown::new(PARTICLE_LIFETIME),
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
