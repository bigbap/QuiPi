use quipi_2d::{
    components::{
        CQuad, CTransform2D
    }, resources::RTileMap, schemas::{
        ISchema, SchemaSprite
    }
};
use quipi_core::{
    FrameResponse,
    FrameState,
    IController,
    Registry,
    VersionedIndex
};
use sdl2::{event::Event, keyboard::Keycode};

// const SPEED: f32 = 0.5;

pub struct PlayerController {
    pub player: VersionedIndex,
    // velocity: glm::Vec2,
    tile_map: u64,
    tile: glm::Vec2
}

impl PlayerController {
    pub fn new(
        registry: &mut Registry,
        tile_map: u64
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let r_tile_map = registry.get_resource::<RTileMap>(tile_map).unwrap();
        let mut this_schema = SchemaSprite::default();
        let start_tile = glm::vec2(10.0, 10.0);

        let transform = CTransform2D {
            translate: r_tile_map.get_tile_pos((
                start_tile.x as usize,
                start_tile.y as usize
            )).xy(),
            scale: glm::vec2(1.0, 1.0),
            ..CTransform2D::default()
        };
        let quad = CQuad {
            width: 32.0,
            height: 32.0,
            ..CQuad::default()
        };

        this_schema.transform = transform;
        this_schema.quad = quad;
        this_schema.tag = "sprite".into();
        this_schema.texture = Some("Player.png".into());

        let id = this_schema.build_entity(registry)?;

        Ok(Self {
            player: id,
            // velocity: glm::vec2(0.0, 0.0),
            tile: start_tile,
            tile_map
        })
    }
}

impl IController for PlayerController {
    fn update(&mut self, frame_state: &mut FrameState, registry: &mut Registry) -> FrameResponse {
        for event in frame_state.events.iter() {
            match event {
                Event::KeyDown { keycode, repeat: false, .. } => {
                    // match keycode {
                    //     Some(Keycode::W) => self.velocity.y += 1.0,
                    //     Some(Keycode::S) => self.velocity.y -= 1.0,
                    //     Some(Keycode::A) => self.velocity.x -= 1.0,
                    //     Some(Keycode::D) => self.velocity.x += 1.0,
                    //     _ => ()
                    // }

                    match keycode {
                        Some(Keycode::W) => self.tile.x += 1.0,
                        Some(Keycode::S) => self.tile.x -= 1.0,
                        Some(Keycode::A) => self.tile.y -= 1.0,
                        Some(Keycode::D) => self.tile.y += 1.0,
                        _ => ()
                    }
                },
                // Event::KeyUp { keycode, repeat: false, .. } => {
                //     match keycode {
                //         Some(Keycode::W) => self.velocity.y -= 1.0,
                //         Some(Keycode::S) => self.velocity.y += 1.0,
                //         Some(Keycode::A) => self.velocity.x += 1.0,
                //         Some(Keycode::D) => self.velocity.x -= 1.0,
                //         _ => ()
                //     }
                // },
                _ => ()
            };
        }

        let tile_map = registry.get_resource::<RTileMap>(self.tile_map).unwrap();
        let new_translate = tile_map.get_tile_pos((
            self.tile.x as usize,
            self.tile.y as usize
        )).xy();
        if let Some(transform) = registry.entities.get_mut::<CTransform2D>(&self.player) {
            transform.translate = new_translate;
            // let mut velocity = glm::vec2(self.velocity.x, self.velocity.y);
            // if velocity.x != 0.0 && self.velocity.y != 0.0 {
            //     velocity = glm::normalize(&velocity);
            // }
            
            // transform.translate.x += velocity.x * SPEED;
            // transform.translate.y += velocity.y * SPEED;
        }

        FrameResponse::None
    }
}