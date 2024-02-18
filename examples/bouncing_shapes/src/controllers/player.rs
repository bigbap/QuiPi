use quipi_2d::{
    components::{
        CQuad, CTransform2D
    },
    schemas::{
        ISchema, SchemaSprite
    }
};
use quipi_core::{core::canvas::get_dimensions, FrameResponse, FrameState, IController, Registry, VersionedIndex};
use sdl2::{event::Event, keyboard::Keycode};

const TAG: &str = "bubble"; // trick the bubble controller to render us 
const CAMERA: &str = "main_camera";
const SHADER: &str = "sprite";
const SPEED: f32 = 3.0;

pub struct PlayerController {
    pub player: VersionedIndex,
    velocity: glm::Vec2,
}

impl PlayerController {
    pub fn new(registry: &mut Registry) -> Result<Self, Box<dyn std::error::Error>> {
        let mut this_schema = SchemaSprite::default();

        let (_x, _y, width, height) = get_dimensions();

        let transform = CTransform2D {
            translate: glm::vec2(
                width as f32 / 2.0,
                height as f32 / 2.0
            ),
            scale: glm::vec2(1.0, 1.0),
            ..CTransform2D::default()
        };
        let quad = CQuad {
            width: 64.0,
            height: 64.0,
            ..CQuad::default()
        };

        this_schema.transform = transform;
        this_schema.quad = quad;
        this_schema.tag = TAG.into();
        this_schema.shader = SHADER.into();
        this_schema.texture = Some("Player.png".into());
        this_schema.camera = CAMERA.to_string();

        let id = this_schema.build_entity(registry)?;

        Ok(Self {
            player: id,
            velocity: glm::vec2(0.0, 0.0),
        })
    }
}

impl IController for PlayerController {
    fn update(&mut self, frame_state: &mut FrameState, registry: &mut Registry) -> FrameResponse {
        for event in frame_state.events.iter() {
            match event {
                Event::KeyDown { keycode, repeat: false, .. } => {
                    match keycode {
                        Some(Keycode::W) => self.velocity.y += SPEED,
                        Some(Keycode::S) => self.velocity.y -= SPEED,
                        Some(Keycode::A) => self.velocity.x -= SPEED,
                        Some(Keycode::D) => self.velocity.x += SPEED,
                        _ => ()
                    }
                },
                Event::KeyUp { keycode, repeat: false, .. } => {
                    match keycode {
                        Some(Keycode::W) => self.velocity.y -= SPEED,
                        Some(Keycode::S) => self.velocity.y += SPEED,
                        Some(Keycode::A) => self.velocity.x += SPEED,
                        Some(Keycode::D) => self.velocity.x -= SPEED,
                        _ => ()
                    }
                },
                _ => ()
            };
        }

        if let Some(transform) = registry.entities.get_mut::<CTransform2D>(&self.player) {
            transform.translate.x += self.velocity.x;
            transform.translate.y += self.velocity.y;
        }

        FrameResponse::None
    }
}