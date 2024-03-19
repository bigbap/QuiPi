extern crate nalgebra_glm as glm;
extern crate quipi;

// mod particle_system;
mod asteroid;
mod camera;
mod ship;
mod stars;

use asteroid::Asteroids;
use camera::Camera;
pub use quipi::prelude::*;
use quipi::{
    assets::{AssetHandle, AssetId, AssetServer, Assets, Source},
    core::prelude::{
        random::Random,
        trig::{magnitude2d_squared, rotate2d},
        Countdown, Interval,
    },
    gfx::{
        prelude::quad::{DefaultQuadShader, QUAD_SHADER_NAME},
        render::{
            assets::{Texture, TextureLoader},
            cameras::{camera_bundle, CameraMetadata},
            renderers::quads::RenderQuads,
            viewport::Viewport,
            RenderBasePlugin,
        },
    },
    prelude::*,
    resources::{AsAny, Resource},
    QPResult,
};

use ship::Ship;
use stars::Stars;

pub static WIDTH: u32 = 1600;
pub static HEIGHT: u32 = 900;

fn main() {
    if let Err(e) = run() {
        eprintln!("Space Shooter ended unexpectedly: {}", e);
    }
}

pub fn run() -> Result<(), QPError> {
    // let mut app = App::init("Space Shooter", WIDTH, HEIGHT, 348756)?;

    // app.world.registry.asset_manager.load_asset(
    //     "shader",
    //     RShader::from_str(
    //         SPRITE_VERT,
    //         SPRITE_FRAG,
    //         vec![
    //             ShaderUniforms::ViewMatrix("view".into()),
    //             ShaderUniforms::ProjectionMatrix("projection".into()),
    //         ],
    //     )?,
    // )?;

    // app.world.registry.asset_manager.load_asset(
    //     "space_tilesheet",
    //     qp_assets::RTexture {
    //         texture: qp_gfx::texture::from_image("assets/textures/space.png")?,
    //         texture_dims: glm::vec2(8.0, 6.0),
    //     },
    // )?;

    // let game = GameController::new(&mut app)?;
    // app.register_controller(game);

    // let renderer = SpriteRenderer::new(&mut app.world.registry, "camera", "shader")?;
    // app.register_renderer(renderer);

    // app.run((0.1, 0.1, 0.1, 1.0))

    App::new()
        .add_resource(ClearColor::new((0.1, 0.1, 0.1, 1.0)))
        .add_plugins(default_plugins("Space Shooter", WIDTH, HEIGHT))
        .add_plugins(RenderBasePlugin)
        .add_plugins(DefaultQuadShader)
        .add_plugins(ParticleSystem)
        .add_plugins(SetupSpaceShooter)
        .add_plugins(Camera)
        .add_plugins(Ship)
        .add_plugins(Stars)
        .add_plugins(Asteroids)
        .run()
}

struct SetupSpaceShooter;
impl Plugin for SetupSpaceShooter {
    fn build(&self, app: &mut App) -> QPResult<()> {
        app.add_system(Startup, setup);
        app.add_system(Update, update);

        let game_state = GameState {
            texture_handle: None,
            timer: Timer::new(),
            delta: 0.0,
            score_interval: Interval::new(500),
            asteroid_spawn_interval: Interval::new(500),
            bullet_spawn_interval: Interval::new(500),
            game_over: false,
            firing: false,
        };
        app.add_resource(game_state);

        Ok(())
    }
}

#[derive(Resource, AsAny)]
struct GameState {
    pub texture_handle: Option<AssetHandle<Texture>>,
    pub timer: Timer,
    pub delta: f32,
    pub score_interval: Interval,
    pub asteroid_spawn_interval: Interval,
    pub bullet_spawn_interval: Interval,
    pub game_over: bool,
    pub firing: bool,
}

fn setup(
    asset_server: ResMut<AssetServer>,
    interner: ResMut<StringInterner>,
    textures: ResMut<Assets<Texture>>,
    game_state: ResMut<GameState>,
) {
    let asset_server = asset_server.unwrap();
    let interner = interner.unwrap();
    let textures = textures.unwrap();
    let game_state = game_state.unwrap();

    let path = "assets/textures/space.png";
    let texture = asset_server
        .load(TextureLoader {
            source: Source::Path(path),
            dims: Some((8, 6)),
        })
        .unwrap();

    let id = interner.intern(path);
    game_state.texture_handle = Some(textures.add(id, texture));
}

fn update(state: ResMut<GameState>, storage: Res<StorageManager>) {
    let (Some(state), Some(storage)) = (state, storage) else {
        return;
    };

    // ONLY UPDATE THE DELTA HERE
    state.delta = state.timer.delta() as f32 / 1000.0;

    let entities = storage.get(StorageId::Entities).unwrap().len();
    // println!("entities: {}", entities);
}

// pub struct GameController {
//     rand: Random,

//     score: Score,
//     score_interval: Interval,
//     game_over_text: GameOver,
//     ship: ship::Ship,
//     bullets: Vec<Bullet>,
//     asteroids: Vec<Asteroid>,
//     stars: Vec<Star>,

//     asteroid_spawn_interval: Interval,
//     bullet_spawn_interval: Interval,
//     star_spawn_interval: Interval,

//     game_over: bool,
//     firing: bool,
// }

// impl GameController {
//     pub fn new(app: &mut App) -> Result<Self, QPError> {
//         let font = app
//             .world
//             .registry
//             .asset_manager
//             .load_asset("Poppins-Regular", qp_assets::RFont::new("Poppins-Regular")?)?;

//         let ship = ship::Ship::new(&mut app.world)?;
//         let camera = Camera::new(&mut app.world, ship.index)?;
//         let score = Score::new(font)?;
//         let game_over_text = GameOver::new(font)?;

//         app.register_controller(camera);

//         if cfg!(debug_assertions) {
//             let text = DebugInfoText::new(font)?;
//             app.register_controller(text);
//         }

//         let rand = Random::from_seed(1234);

//         Ok(Self {
//             rand,
//             score,
//             game_over_text,
//             ship,
//             bullets: vec![],
//             asteroids: vec![],
//             stars: vec![],
//             score_interval: Interval::new(0.5),
//             asteroid_spawn_interval: Interval::new(0.5),
//             bullet_spawn_interval: Interval::new(0.2),
//             star_spawn_interval: Interval::new(0.002),
//             game_over: false,
//             firing: false,
//         })
//     }

//     fn reset(&mut self, registry: &mut GlobalRegistry) {
//         for bullet in self.bullets.iter() {
//             registry.entity_manager.set_to_delete(bullet.index);
//         }
//         for asteroid in self.asteroids.iter() {
//             registry.entity_manager.set_to_delete(asteroid.index);
//         }

//         self.bullets.clear();
//         self.asteroids.clear();

//         self.asteroid_spawn_interval.check();
//         self.bullet_spawn_interval.check();

//         self.score.score = 0;
//         self.game_over = false;
//     }

//     fn spawn_bullet(&mut self, registry: &mut GlobalRegistry) -> Result<(), QPError> {
//         if !self.bullet_spawn_interval.check() || !self.firing {
//             return Ok(());
//         }

//         let Some(ship) = registry
//             .entity_manager
//             .get::<CTransform2D>(&self.ship.index)
//         else {
//             return Ok(());
//         };
//         let direction = ship.direction();
//         let position = ship.translate + (direction * 28.0);
//         let bullet = Bullet::new(registry, position, direction, ship.rotate);

//         self.bullets.push(bullet?);

//         Ok(())
//     }

//     fn spawn_asteroid(&mut self, world: &mut World) -> Result<(), QPError> {
//         if !self.asteroid_spawn_interval.check() {
//             return Ok(());
//         }

//         let (_x, _y, width, height) = world.viewport.get_dimensions();

//         let ship = world
//             .registry
//             .entity_manager
//             .get::<CTransform2D>(&self.ship.index)
//             .unwrap();

//         let ship_pos = ship.translate;
//         let offset = 100.0;
//         let pos_local = glm::vec2((width / 2) as f32 + offset, (height / 2) as f32 + offset);
//         let pos_local = rotate2d(&pos_local, self.rand.random() * 2.0 * glm::pi::<f32>());
//         let pos = ship_pos + pos_local;

//         let asteroid = Asteroid::new(
//             &mut world.registry,
//             ship_pos,
//             pos,
//             (self.rand.random() + 1.0) * 2.0,
//             self.rand.random() * 2.0 * glm::pi::<f32>(),
//             &mut world.rand,
//         )?;
//         self.asteroids.push(asteroid);

//         Ok(())
//     }

//     fn spawn_star(&mut self, world: &mut World) -> Result<(), QPError> {
//         if !self.star_spawn_interval.check() {
//             return Ok(());
//         }

//         let ship_transform = world
//             .registry
//             .entity_manager
//             .get::<CTransform2D>(&self.ship.index)
//             .unwrap();

//         let ship_pos = ship_transform.translate;
//         let star = Star::new(world, ship_pos)?;

//         self.stars.push(star);

//         Ok(())
//     }
// }

// impl Controller for GameController {
//     fn update(&mut self, world: &mut World) -> FrameResult {
//         for event in world.events.iter() {
//             match event {
//                 Event::Quit { .. } => {
//                     return FrameResult::Quit;
//                 }
//                 Event::KeyDown {
//                     keycode: Some(Keycode::Escape),
//                     ..
//                 } => {
//                     return FrameResult::Quit;
//                 }
//                 Event::KeyDown {
//                     keycode: Some(Keycode::Space),
//                     repeat: false,
//                     ..
//                 } => {
//                     if !self.game_over {
//                         self.firing = true;
//                     }
//                 }
//                 Event::KeyDown {
//                     keycode: Some(Keycode::Return),
//                     repeat: false,
//                     ..
//                 } => {
//                     if self.game_over {
//                         self.reset(&mut world.registry);
//                     }
//                 }
//                 Event::KeyUp {
//                     keycode: Some(Keycode::Space),
//                     ..
//                 } => {
//                     self.firing = false;
//                 }
//                 _ => (),
//             };
//         }

//         self.score.update(world);

//         if self.game_over {
//             self.game_over_text.update(world);

//             return FrameResult::None;
//         }

//         for asteroid in self.asteroids.iter_mut() {
//             if !asteroid.alive {
//                 continue;
//             }

//             // check for collision with ship
//             let ship_transform = world
//                 .registry
//                 .entity_manager
//                 .get::<CTransform2D>(&self.ship.index)
//                 .unwrap()
//                 .clone();

//             if asteroid.check_collision(&mut world.registry, &ship_transform, 32.0) {
//                 self.game_over = true;
//             };

//             // check for collision with bullet
//             for bullet in self.bullets.iter_mut() {
//                 if !bullet.alive {
//                     continue;
//                 }

//                 let bullet_transform = world
//                     .registry
//                     .entity_manager
//                     .get::<CTransform2D>(&bullet.index)
//                     .unwrap()
//                     .clone();

//                 if asteroid.check_collision(&mut world.registry, &bullet_transform, 16.0) {
//                     self.score.score += 5;
//                     bullet.destroy(&mut world.registry);
//                 };
//             }

//             asteroid.update(world);
//         }

//         for bullet in self.bullets.iter_mut() {
//             bullet.update(world);
//         }

//         for star in self.stars.iter_mut() {
//             star.update(world);
//         }

//         self.spawn_asteroid(world).unwrap();
//         self.spawn_bullet(&mut world.registry).unwrap();
//         self.spawn_star(world).unwrap();

//         self.ship.update(world);

//         if self.score_interval.check() {
//             self.score.score += 1;
//         }

//         FrameResult::None
//     }
// }

// struct Asteroid {
//     index: Index,
//     rotation_step: f32,
//     scale: f32,
//     alive: bool,
//     countdown: Countdown,
// }

// impl Asteroid {
//     pub fn new(
//         registry: &mut GlobalRegistry,
//         ship_pos: glm::Vec2,
//         position: glm::Vec2,
//         scale: f32,
//         rotate: f32,
//         rand: &mut Random,
//     ) -> Result<Self, QPError> {
//         let texture_id = registry
//             .asset_manager
//             .get_asset_id("space_tilesheet")
//             .ok_or(QPError::SpriteTextureDoesntExist)?;

//         let texture = registry
//             .asset_manager
//             .get::<qp_assets::RTexture>(texture_id)
//             .ok_or(QPError::SpriteTextureDoesntExist)?;

//         let quad = CQuad {
//             width: 32.0,
//             height: 32.0,
//             ..CQuad::default()
//         };

//         let direction = (ship_pos - position).normalize();

//         let index = registry.entity_manager.create((
//             CTag {
//                 tag: "asteroid".to_string(),
//             },
//             CTransform2D {
//                 translate: position,
//                 rotate,
//                 scale: glm::vec2(scale, scale),
//                 ..CTransform2D::default()
//             },
//             CVelocity2D {
//                 x: direction.x * rand.range(150, 300) as f32,
//                 y: direction.y * rand.range(150, 300) as f32,
//             },
//             CSprite::new(
//                 &quad,
//                 Some(glm::vec4(0.9, 0.9, 0.9, 1.0)),
//                 Some(TextureAtlas {
//                     texture: texture_id,
//                     texture_dims: texture.texture_dims,
//                     active_texture: glm::vec2(0.0, 1.0),
//                 }),
//             ),
//         ));

//         Ok(Self {
//             index,
//             rotation_step: (rand.random() + 0.7) * 2.0,
//             alive: true,
//             scale,
//             countdown: Countdown::new(30.0),
//         })
//     }

//     pub fn check_collision(
//         &mut self,
//         registry: &mut GlobalRegistry,
//         obj: &CTransform2D,
//         obj_radius: f32,
//     ) -> bool {
//         let transform = registry
//             .entity_manager
//             .get::<CTransform2D>(&self.index)
//             .unwrap();

//         let offset = 25.0;
//         let threshold = obj_radius + (self.scale * 16.0) - offset;
//         if magnitude2d_squared(&transform.translate, &obj.translate) < threshold.powf(2.0) {
//             registry.entity_manager.set_to_delete(self.index);
//             self.alive = false;

//             return true;
//         }

//         false
//     }

//     pub fn destroy(&mut self, registry: &mut GlobalRegistry) {
//         registry.entity_manager.set_to_delete(self.index);
//         self.alive = false;
//     }
// }

// impl Controller for Asteroid {
//     fn update(&mut self, world: &mut World) -> FrameResult {
//         let time_left = self.countdown.check();

//         if time_left == 0.0 {
//             self.destroy(&mut world.registry);

//             return FrameResult::None;
//         }

//         let velocity = world
//             .registry
//             .entity_manager
//             .get::<CVelocity2D>(&self.index)
//             .unwrap()
//             .clone();

//         if let Some(transform) = world
//             .registry
//             .entity_manager
//             .get_mut::<CTransform2D>(&self.index)
//         {
//             transform.translate.x += velocity.x * world.delta;
//             transform.translate.y += velocity.y * world.delta;

//             transform.rotate += world.delta * self.rotation_step;
//         }

//         let sprite = world
//             .registry
//             .entity_manager
//             .get_mut::<CSprite>(&self.index)
//             .unwrap();

//         sprite.color.y = time_left / self.countdown.countdown;
//         sprite.color.z = time_left / self.countdown.countdown;
//         sprite.color.w = time_left / self.countdown.countdown;

//         FrameResult::None
//     }
// }

// struct Score {
//     score: u32,
//     font: u64,
// }

// impl Score {
//     pub fn new(font: u64) -> Result<Self, QPError> {
//         Ok(Self { score: 0, font })
//     }
// }

// impl Controller for Score {
//     fn update(&mut self, world: &mut World) -> FrameResult {
//         let (_x, _y, _width, height) = world.viewport.get_dimensions();
//         world.text_buffer.push(qp_gfx::QPText {
//             text: format!("score: {}", self.score),
//             pos: glm::vec2(20.0, height as f32 - 40.0),
//             style: qp_gfx::QPTextStyle {
//                 font: self.font,
//                 color: glm::vec4(1.0, 1.0, 1.0, 1.0),
//                 scale: 0.4,
//             },
//         });

//         FrameResult::None
//     }
// }

// struct GameOver {
//     font: u64,
// }

// impl GameOver {
//     pub fn new(font: u64) -> Result<Self, QPError> {
//         Ok(Self { font })
//     }
// }

// impl Controller for GameOver {
//     fn update(&mut self, world: &mut World) -> FrameResult {
//         let (_x, _y, width, height) = world.viewport.get_dimensions();
//         world.text_buffer.push(qp_gfx::QPText {
//             text: "Game Over".into(),
//             pos: glm::vec2((width as f32 / 2.0) - 300.0, height as f32 / 2.0),
//             style: qp_gfx::QPTextStyle {
//                 font: self.font,
//                 color: glm::vec4(1.0, 1.0, 1.0, 0.6),
//                 scale: 2.0,
//             },
//         });
//         world.text_buffer.push(qp_gfx::QPText {
//             text: "Press Enter to start again".into(),
//             pos: glm::vec2((width as f32 / 2.0) - 160.0, (height as f32 / 2.0) - 50.0),
//             style: qp_gfx::QPTextStyle {
//                 font: self.font,
//                 color: glm::vec4(0.8, 0.8, 0.8, 1.0),
//                 scale: 0.5,
//             },
//         });

//         FrameResult::None
//     }
// }

// struct DebugInfoText {
//     font: u64,
// }

// impl DebugInfoText {
//     pub fn new(font: u64) -> Result<Self, QPError> {
//         Ok(Self { font })
//     }
// }

// impl Controller for DebugInfoText {
//     fn update(&mut self, world: &mut World) -> FrameResult {
//         let entity_count = world.registry.entity_manager.count();
//         world.text_buffer.push(qp_gfx::QPText {
//             text: format!("entities: {}", entity_count),
//             pos: glm::vec2(20.0, 20.0),
//             style: qp_gfx::QPTextStyle {
//                 font: self.font,
//                 color: glm::vec4(1.0, 1.0, 1.0, 1.0),
//                 scale: 0.4,
//             },
//         });

//         world.text_buffer.push(qp_gfx::QPText {
//             text: format!("draw calls: {}", world.debug_info.draw_calls),
//             pos: glm::vec2(20.0, 40.0),
//             style: qp_gfx::QPTextStyle {
//                 font: self.font,
//                 color: glm::vec4(1.0, 1.0, 1.0, 1.0),
//                 scale: 0.4,
//             },
//         });

//         FrameResult::None
//     }
// }
