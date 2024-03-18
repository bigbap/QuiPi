use qp_common::components::*;
use qp_core::{trig::pi, Countdown};
use quipi::{prelude::*, schemas::sprite::TextureAtlas};
use sdl2::keyboard::Keycode;

use crate::particle_system::{Particle, ParticleSystem};

pub struct Ship {
    pub index: Index,
    particle_system: ParticleSystem<ExhaustParticle>,

    mouse_pos: glm::Vec2,
    acceleration: f32,
    max_velocity: f32,
    thrust: bool,
}

impl Ship {
    pub fn new(world: &mut World) -> Result<Self, QPError> {
        let texture_id = world
            .registry
            .asset_manager
            .get_asset_id("space_tilesheet")
            .ok_or(QPError::SpriteTextureDoesntExist)?;

        let texture = world
            .registry
            .asset_manager
            .get::<qp_assets::RTexture>(texture_id)
            .ok_or(QPError::SpriteTextureDoesntExist)?;

        let index = world.registry.entity_manager.create((
            CTag {
                tag: "ship".to_string(),
            },
            CVelocity2D { x: 0.0, y: 0.0 },
            CTransform2D {
                translate: glm::vec2(0.0, 0.0),
                ..CTransform2D::default()
            },
            CSprite::new(
                &CQuad {
                    width: 64.0,
                    height: 64.0,
                    center_x: 0.0,
                    center_y: 0.0,
                },
                Some(glm::vec4(0.8, 0.2, 0.0, 1.0)),
                Some(TextureAtlas {
                    texture: texture_id,
                    texture_dims: texture.texture_dims,
                    active_texture: glm::vec2(1.0, 3.0),
                }),
            ),
        ));

        let particle_system = ParticleSystem::new(0.02, false, move |world: &mut World| {
            let Some(ship) = world.registry.entity_manager.get::<CTransform2D>(&index) else {
                return None;
            };

            Some(ExhaustParticle::new(world, &ship.clone()).unwrap())
        });

        Ok(Self {
            index,
            acceleration: 100.0,
            max_velocity: 500.0,
            thrust: false,
            mouse_pos: glm::vec2(0.0, 0.0),
            particle_system,
        })
    }

    fn rotate_ship(&mut self, world: &mut World) {
        let (_x, _y, width, height) = world.viewport.get_dimensions();
        let x = self.mouse_pos.x - width as f32 / 2.0;
        let y = (self.mouse_pos.y - height as f32 / 2.0) * -1.0;

        let angle =
            qp_core::trig::angle(&glm::vec3(0.0, 1.0, 0.0), &glm::vec3(x, y, 0.0)) - (pi() / 2.0);

        if let Some(transform) = world
            .registry
            .entity_manager
            .get_mut::<CTransform2D>(&self.index)
        {
            transform.rotate = angle;
        }
    }
}

impl Controller for Ship {
    fn update(&mut self, world: &mut World) -> FrameResult {
        for event in world.events.iter() {
            match event {
                Event::MouseMotion { x, y, .. } => {
                    self.mouse_pos = glm::vec2(*x as f32, *y as f32);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    repeat: false,
                    ..
                } => {
                    self.thrust = true;
                    self.particle_system.active = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    repeat: false,
                    ..
                } => {
                    self.thrust = false;
                    self.particle_system.active = false;
                }
                _ => (),
            };
        }

        self.rotate_ship(world);
        self.particle_system.update(world).unwrap();

        // calculate velocity based on direction and acceleration
        let Some(velocity) = world
            .registry
            .entity_manager
            .get_mut::<CVelocity2D>(&self.index)
        else {
            return FrameResult::None;
        };

        if !self.thrust && velocity.x <= 0.0 && velocity.y <= 0.0 {
            return FrameResult::None;
        }

        let acceleration = world.delta
            * match self.thrust {
                true => self.acceleration,
                false => -self.acceleration,
            };

        velocity.x = (velocity.x + acceleration).clamp(0.0, self.max_velocity);
        velocity.y = (velocity.y + acceleration).clamp(0.0, self.max_velocity);

        // apply velocity to translation
        let Some(velocity) = world
            .registry
            .entity_manager
            .get::<CVelocity2D>(&self.index)
        else {
            return FrameResult::None;
        };

        let velocity = velocity.clone();

        if let Some(transform) = world
            .registry
            .entity_manager
            .get_mut::<CTransform2D>(&self.index)
        {
            let direction = transform.direction();
            transform.translate.x += velocity.x * world.delta * direction.x;
            transform.translate.y += velocity.y * world.delta * direction.y;
        }

        FrameResult::None
    }
}

struct ExhaustParticle {
    index: Index,
    alive: bool,
    countdown: Countdown,
}

impl ExhaustParticle {
    pub fn new(world: &mut World, ship_transform: &CTransform2D) -> Result<Self, QPError> {
        let texture_id = world
            .registry
            .asset_manager
            .get_asset_id("space_tilesheet")
            .ok_or(QPError::SpriteTextureDoesntExist)?;

        let texture = world
            .registry
            .asset_manager
            .get::<qp_assets::RTexture>(texture_id)
            .ok_or(QPError::SpriteTextureDoesntExist)?;

        let quad = CQuad {
            width: 64.0,
            height: 64.0,
            center_x: 0.0,
            center_y: 0.0,
        };

        let offset = ship_transform.direction() * 28.0;
        let scale_factor = world.rand.random() + 1.0;
        let index = world.registry.entity_manager.create((
            CTag {
                tag: "particle".to_string(),
            },
            CTransform2D {
                translate: ship_transform.translate - offset,
                rotate: world.rand.random() * 2.0 * glm::pi::<f32>(),
                scale: glm::vec2(scale_factor, scale_factor),
                ..CTransform2D::default()
            },
            CSprite::new(
                &quad,
                Some(glm::vec4(1.0, 0.7, 0.2, 1.0)),
                Some(TextureAtlas {
                    texture: texture_id,
                    texture_dims: texture.texture_dims,
                    active_texture: glm::vec2(4.0, 2.0),
                }),
            ),
        ));

        Ok(Self {
            index,
            alive: true,
            countdown: Countdown::new(0.4),
        })
    }
}

impl Particle for ExhaustParticle {
    fn update(&mut self, world: &mut World) {
        if !self.alive {
            return;
        }

        let time_left = self.countdown.check();

        if time_left == 0.0 {
            self.alive = false;
            world.registry.entity_manager.set_to_delete(self.index);

            return;
        }

        let sprite = world
            .registry
            .entity_manager
            .get_mut::<CSprite>(&self.index)
            .unwrap();

        sprite.color.w = time_left;
    }
}
