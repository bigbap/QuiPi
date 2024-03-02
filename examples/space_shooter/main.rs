extern crate nalgebra_glm as glm;
extern crate quipi;

pub use quipi::prelude::*;
use quipi::{
    asset_manager::assets::{camera::OrthographicCameraParams, RCamera2D, RShader},
    core::prelude::Timer,
    ecs::prelude::components::CTransform2D,
    gfx::prelude::{ShaderUniforms, SpriteRenderer, SPRITE_FRAG, SPRITE_VERT},
    schemas::sprite::TextureAtlas,
};

use qp_ecs::components::*;
use sdl2::{keyboard::Keycode, mouse::MouseButton, sys::KeyCode};

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

pub fn run() -> Result<(), QPError> {
    let mut app = App::init("Space Shooter", WIDTH, HEIGHT)?;

    // app.winapi.relative_mouse_mode(true);

    app.world.registry.asset_manager.load_asset(
        "shader",
        RShader::from_str(
            SPRITE_VERT,
            SPRITE_FRAG,
            vec![
                ShaderUniforms::ViewMatrix("view".into()),
                ShaderUniforms::ProjectionMatrix("projection".into()),
            ],
        )?,
    )?;

    app.world.registry.asset_manager.load_asset(
        "space_tilesheet",
        qp_assets::RTexture {
            texture: qp_gfx::texture::from_image("assets/textures/space.png")?,
            texture_dims: glm::vec2(8.0, 6.0),
        },
    )?;

    let ship = Ship::new(&mut app.world)?;
    let bullets = Bullets::new(ship.index);
    let camera = Camera::new(&mut app.world, ship.index)?;

    app.register_controller(GameController {});
    app.register_controller(ship);
    app.register_controller(bullets);
    app.register_controller(camera);

    let renderer = SpriteRenderer::new(&mut app.world.registry, "camera", "shader")?;
    app.register_renderer(renderer);

    app.run((0.1, 0.1, 0.1, 1.0))
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Space Shooter ended unexpectedly: {}", e);
    }
}

pub struct GameController {}

impl Controller for GameController {
    fn update(&mut self, world: &mut World) -> FrameResult {
        for event in world.events.iter() {
            match event {
                Event::Quit { .. } => {
                    return FrameResult::Quit;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return FrameResult::Quit;
                }
                _ => (),
            };
        }

        FrameResult::None
    }
}

pub struct Camera {
    ship: VersionedIndex,
    id: u64,
}

impl Camera {
    pub fn new(world: &mut World, ship: VersionedIndex) -> Result<Self, QPError> {
        let id = world.registry.asset_manager.load_asset(
            "camera",
            RCamera2D::new(
                OrthographicCameraParams {
                    right: WIDTH as f32,
                    top: HEIGHT as f32,
                    ..OrthographicCameraParams::default()
                },
                1.0,
                CTransform2D::default(),
            ),
        )?;

        Ok(Self { ship, id })
    }
}

impl Controller for Camera {
    fn update(&mut self, world: &mut World) -> FrameResult {
        let Some(transform) = world
            .registry
            .entity_manager
            .get::<CTransform2D>(&self.ship)
        else {
            return FrameResult::None;
        };
        let Some(camera) = world.registry.asset_manager.get_mut::<RCamera2D>(self.id) else {
            return FrameResult::None;
        };

        camera.follow(transform);

        // println!("entity count: {}", world.registry.entity_manager.count());

        FrameResult::None
    }
}

pub struct Ship {
    pub index: VersionedIndex,
    texture: u64,
    texture_index: glm::Vec2,
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

        let quad = CQuad {
            width: 64.0,
            height: 64.0,
            center_x: 0.0,
            center_y: 0.0,
        };

        let index = EntityBuilder::create(&mut world.registry.entity_manager)
            .with(CVelocity2D { x: 10.0, y: 10.0 })
            .with(CTransform2D {
                translate: glm::vec2(0.0, 0.0),
                ..CTransform2D::default()
            })
            .with(CSprite::new(
                &quad,
                Some(glm::vec4(1.0, 1.0, 1.0, 1.0)),
                Some(TextureAtlas {
                    texture: texture_id,
                    texture_dims: texture.texture_dims,
                    active_texture: glm::vec2(6.0, 5.0),
                }),
            ))
            .build();

        Ok(Self {
            index,
            texture: texture_id,
            texture_index: glm::vec2(0.0, 0.0),
        })
    }
}

impl Controller for Ship {
    fn update(&mut self, world: &mut World) -> FrameResult {
        let (_x, _y, width, height) = world.viewport.get_dimensions();

        for event in world.events.iter() {
            match event {
                Event::MouseMotion { x, y, .. } => {
                    let x = x - width / 2;
                    let y = (y - height / 2) * -1;
                    let angle = qp_core::trig::angle(
                        &glm::vec3(0.0, 1.0, 0.0),
                        &glm::vec3(x as f32, y as f32, 0.0),
                    );

                    if let Some(transform) = world
                        .registry
                        .entity_manager
                        .get_mut::<CTransform2D>(&self.index)
                    {
                        transform.rotate = match x > 0 {
                            true => {
                                let angle = (2.0 * glm::pi::<f32>()) - angle;

                                angle
                            }
                            false => angle,
                        };
                    }
                }
                _ => (),
            };
        }

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
            transform.translate.x += velocity.x * world.delta;
            transform.translate.y += velocity.y * world.delta;

            // println!("position: {:?}", transform.translate);
        }

        FrameResult::None
    }
}

pub struct Bullets {
    ship: VersionedIndex,
    instances: Vec<Bullet>,
}

impl Bullets {
    pub fn new(ship: VersionedIndex) -> Self {
        Self {
            ship,
            instances: vec![],
        }
    }

    pub fn spawn(&mut self, registry: &mut GlobalRegistry) -> Result<(), QPError> {
        let Some(ship) = registry.entity_manager.get::<CTransform2D>(&self.ship) else {
            return Ok(());
        };

        let bullet = Bullet::new(registry, ship.translate);

        self.instances.push(bullet?);

        Ok(())
    }
}

impl Controller for Bullets {
    fn update(&mut self, world: &mut World) -> FrameResult {
        for instance in self.instances.iter_mut() {
            instance.update(world);
        }

        for event in world.events.iter() {
            match event {
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => self.spawn(&mut world.registry).unwrap(),
                _ => (),
            };
        }

        FrameResult::None
    }
}

pub struct Bullet {
    index: VersionedIndex,
    timer: Timer,
}

impl Bullet {
    pub fn new(registry: &mut GlobalRegistry, position: glm::Vec2) -> Result<Self, QPError> {
        let color = glm::vec4(1.0, 1.0, 1.0, 1.0);

        let texture_id = registry
            .asset_manager
            .get_asset_id("space_tilesheet")
            .ok_or(QPError::SpriteTextureDoesntExist)?;

        let texture = registry
            .asset_manager
            .get::<qp_assets::RTexture>(texture_id)
            .ok_or(QPError::SpriteTextureDoesntExist)?;

        let quad = CQuad {
            width: 64.0,
            height: 64.0,
            center_x: 0.0,
            center_y: 0.0,
        };

        let index = EntityBuilder::create(&mut registry.entity_manager)
            .with(CTransform2D {
                translate: position,
                ..CTransform2D::default()
            })
            .with(CVelocity2D { x: 100.0, y: 100.0 })
            .with(CSprite::new(
                &quad,
                Some(glm::vec4(1.0, 1.0, 1.0, 1.0)),
                Some(TextureAtlas {
                    texture: texture_id,
                    texture_dims: texture.texture_dims,
                    active_texture: glm::vec2(4.0, 2.0),
                }),
            ))
            .build();

        let timer = Timer::new();

        Ok(Self { index, timer })
    }

    pub fn update(&mut self, world: &mut World) -> bool {
        let Some(velocity) = world
            .registry
            .entity_manager
            .get::<CVelocity2D>(&self.index)
        else {
            return false;
        };

        let velocity = glm::vec2(velocity.x * world.delta, velocity.y * world.delta);

        if let Some(transform) = world
            .registry
            .entity_manager
            .get_mut::<CTransform2D>(&self.index)
        {
            transform.translate += velocity
        } else {
            return false;
        }

        false
    }
}
