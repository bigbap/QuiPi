use crate::{
    qp_assets::RCamera2D,
    qp_core::{now_secs, random::Random},
    qp_ecs::components::{CQuad, CTag, CTransform2D, CVelocity2D},
    qp_schemas::SchemaSprite,
    Controller, FrameResult, Index, QPError, Schema, World,
};
use sdl2::{event::Event, keyboard::Keycode};

pub struct BubbleController {
    bubbles: Vec<Index>,
    camera: u64,
    rand: Random,
}

impl BubbleController {
    pub fn new(world: &mut World, camera: u64) -> Result<Self, QPError> {
        let mut bubbles = world.registry.entity_manager.query(CTag {
            tag: "sprite".to_string(),
        });
        let mut rand = Random::from_seed(now_secs()?);

        // for stress testing
        for _ in 0..1000 {
            bubbles.push(spawn(&mut rand, world)?);
        }

        Ok(Self {
            bubbles,
            camera,
            rand,
        })
    }
}

impl Controller for BubbleController {
    fn update(&mut self, world: &mut World) -> FrameResult {
        if world.debug_mode {
            return FrameResult::None;
        }

        // handle input
        for event in world.events.iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    match spawn(&mut self.rand, world) {
                        Ok(index) => self.bubbles.push(index),
                        Err(_e) => {
                            #[cfg(debug_assertions)]
                            println!(
                                "[bubble controller] there was a problem spawning a bubble: {}",
                                _e.to_string()
                            );
                        }
                    }

                    break;
                }
                _ => (),
            };
        }

        update(&self.bubbles, world, self.camera);

        FrameResult::None
    }
}

fn spawn(rand: &mut Random, world: &mut World) -> Result<Index, QPError> {
    let mut this_schema = SchemaSprite::default();

    let (_x, _y, width, height) = world.viewport.get_dimensions();

    let vel = (rand.range(-200, 200) as f32, rand.range(-200, 200) as f32);
    let color = glm::vec4(rand.random(), rand.random(), rand.random(), 1.0);
    let s_factor = rand.range(5, 25) as f32 / 100.0;
    let transform = CTransform2D {
        translate: glm::vec2(
            // width as f32 / 2.0,
            // height as f32 / 2.0
            rand.range(25, width) as f32,
            rand.range(25, height) as f32,
        ),
        scale: glm::vec2(s_factor, s_factor),
        ..CTransform2D::default()
    };
    let quad = CQuad {
        width: 200.0,
        height: 200.0,
        ..CQuad::default()
    };

    this_schema.velocity = Some(CVelocity2D { x: vel.0, y: vel.1 });
    this_schema.transform = transform;
    this_schema.quad = quad;
    this_schema.color = color;
    this_schema.tag = "sprite".into();
    this_schema.texture = Some("Bubble.png".into());

    let id = this_schema.build_entity(&mut world.registry)?;

    Ok(id)
}

pub fn update(bubbles: &[Index], world: &mut World, camera: u64) {
    let Some(camera) = world.registry.asset_manager.get::<RCamera2D>(camera) else {
        println!("couldn't find camera");

        return;
    };

    let (_x, _y, width, height) = world.viewport.get_dimensions();

    for bubble in bubbles {
        let Some(vel) = world.registry.entity_manager.get::<CVelocity2D>(&bubble) else {
            continue;
        };
        let Some(transform) = world.registry.entity_manager.get::<CTransform2D>(&bubble) else {
            continue;
        };
        let Some(quad) = world.registry.entity_manager.get::<CQuad>(&bubble) else {
            continue;
        };

        let scale = transform.scale;

        let vel = glm::vec2(vel.x, vel.y);
        let translate = transform.translate + (vel * world.delta);
        let w = quad.width * scale.x;
        let h = quad.height * scale.y;
        let (colided_x, colided_y) = check_screen_collision(&camera, translate, w, h);

        let Some(transform) = world
            .registry
            .entity_manager
            .get_mut::<CTransform2D>(&bubble)
        else {
            continue;
        };
        match colided_x {
            -1 => transform.translate.x = 0.0 + w * 0.5,
            1 => transform.translate.x = width as f32 - w * 0.5,
            _ => transform.translate.x = translate.x,
        }
        match colided_y {
            -1 => transform.translate.y = 0.0 + h * 0.5,
            1 => transform.translate.y = height as f32 - h * 0.5,
            _ => transform.translate.y = translate.y,
        }

        let Some(vel) = world
            .registry
            .entity_manager
            .get_mut::<CVelocity2D>(&bubble)
        else {
            continue;
        };
        if colided_x != 0 {
            vel.x *= -1.0
        }
        if colided_y != 0 {
            vel.y *= -1.0
        }
    }
}

fn check_screen_collision(camera: &RCamera2D, pos: glm::Vec2, w: f32, h: f32) -> (i32, i32) {
    let offset_x = w * 0.5;
    let offset_y = h * 0.5;

    let mut colided_x = 0;
    let mut colided_y = 0;

    let cam_width = (camera.params.right - camera.params.left).abs() as f32;
    let cam_height = (camera.params.bottom - camera.params.top).abs() as f32;

    if pos.x <= (0.0 + offset_x) {
        colided_x = -1;
    }
    if pos.x >= (cam_width - offset_x) {
        colided_x = 1;
    }
    if pos.y >= (cam_height - offset_y) {
        colided_y = 1;
    }
    if pos.y <= (0.0 + offset_y) {
        colided_y = -1;
    }

    (colided_x, colided_y)
}
