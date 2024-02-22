use quipi::{
    ecs::components::{
        CQuad,
        CTransform2D,
         CVelocity2D,
         CTag
    },
    schemas::{
        ISchema,
        SchemaSprite
    },
    IController,
    canvas::get_dimensions,
    math::random::Random,
    time::now_secs,
    FrameResponse,
    FrameState,
    Registry,
    VersionedIndex
};
use sdl2::{event::Event, keyboard::Keycode};

pub struct BubbleController {
    bubbles: Vec<VersionedIndex>,
    rand: Random
}

impl BubbleController {
    pub fn new(registry: &mut Registry) -> Result<Self, Box<dyn std::error::Error>> {
        let mut bubbles = registry.entities.query(CTag { tag: "sprite".to_string() });
        let mut rand = Random::from_seed(now_secs()?);

        // for stress testing
        for _ in 0..1000 {
            bubbles.push(spawn(&mut rand, registry)?);
        }

        Ok(Self {
            bubbles,
            rand
        })
    }
}

impl IController for BubbleController {
    fn update(
        &mut self,
        frame_state: &mut FrameState,
        registry: &mut Registry
    ) -> FrameResponse {
        if frame_state.debug_mode { return FrameResponse::None; }

        // handle input
        for event in frame_state.events.iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    match spawn(&mut self.rand, registry) {
                        Ok(index) => self.bubbles.push(index),
                        Err(_e) => {
                            #[cfg(debug_assertions)]
                            println!("[bubble controller] there was a problem spawning a bubble: {}", _e.to_string());
                        }
                    }

                    break;
                },
                _ => ()
            };
        }

        update(&self.bubbles, registry, frame_state);

        FrameResponse::None
    }
}

fn spawn(
    rand: &mut Random,
    registry: &mut Registry,
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    let mut this_schema = SchemaSprite::default();

    let (_x, _y, width, height) = get_dimensions();

    let vel = (
        rand.range(-200, 200) as f32,
        rand.range(-200, 200) as f32,
    );
    let color = glm::vec4(
        rand.random(),
        rand.random(),
        rand.random(),
        1.0
    );
    let s_factor = rand.range(5, 25) as f32 / 100.0;
    let transform = CTransform2D {
        translate: glm::vec2(
            width as f32 / 2.0,
            height as f32 / 2.0
            // rand.range(25, width) as f32,
            // rand.range(25, height) as f32,
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

    let id = this_schema.build_entity(registry)?;

    Ok(id)
}

pub fn update(
    bubbles: &[VersionedIndex],
    registry: &mut Registry,
    frame_state: &mut FrameState
) {
    for bubble in bubbles {
        let Some(vel)       = registry.entities.get::<CVelocity2D>(&bubble)    else { continue };
        let Some(transform) = registry.entities.get::<CTransform2D>(&bubble)   else { continue };
        let Some(quad)      = registry.entities.get::<CQuad>(&bubble)           else { continue };
        
        let scale = transform.scale;

        let vel = glm::vec2(vel.x, vel.y);
        let translate = transform.translate + (vel * frame_state.delta);
        let w = quad.width * scale.x;
        let h = quad.height * scale.y;
        let (colided_x, colided_y) = check_screen_collision(
            translate,
            w,
            h
        );

        let (_x, _y, width, height) = get_dimensions();
        let Some(transform) = registry.entities.get_mut::<CTransform2D>(&bubble) else { continue };
        match colided_x {
            -1 => transform.translate.x = 0.0 + w * 0.5,
            1 => transform.translate.x = width as f32 - w * 0.5,
            _ => transform.translate.x = translate.x
        }
        match colided_y {
            -1 => transform.translate.y = 0.0 + h * 0.5,
            1 => transform.translate.y = height as f32 - h * 0.5,
            _ => transform.translate.y = translate.y
        }

        let Some(vel) = registry.entities.get_mut::<CVelocity2D>(&bubble) else { continue };
        if colided_x != 0 { vel.x *= -1.0 }
        if colided_y != 0 { vel.y *= -1.0 }
    }
}

fn check_screen_collision(
    pos: glm::Vec2,
    w: f32,
    h: f32
) -> (i32, i32) {
    let offset_x = w * 0.5;
    let offset_y = h * 0.5;

    let (_x, _y, width, height) = get_dimensions();

    let mut colided_x = 0;
    let mut colided_y = 0;

    if pos.x <= (0.0 + offset_x) { colided_x = -1; }
    if pos.x >= (width as f32 - offset_x) { colided_x = 1; }
    if pos.y >= (height as f32 - offset_y) { colided_y = 1; }
    if pos.y <= (0.0 + offset_y) {  colided_y = -1; }

    (colided_x, colided_y)
}