use crate::{
    qp_assets::{tilemap::ValidTile, RTileMap},
    qp_schemas::SchemaSprite,
    qp_storage::components::{CQuad, CTransform2D},
    GlobalRegistry, Index, Schema,
};
use quipi::{
    app::{Controller, FrameResult},
    prelude::QPError,
    world::World,
};
use sdl2::{event::Event, keyboard::Keycode};

const PLAYER_SIZE: f32 = 54.0;

pub struct PlayerController {
    pub player: Index,
    tile_map: u64,
    tile: glm::Vec2,
}

impl PlayerController {
    pub fn new(registry: &mut GlobalRegistry, tile_map: u64) -> Result<Self, QPError> {
        let r_tile_map = registry.asset_manager.get::<RTileMap>(tile_map).unwrap();
        let mut this_schema = SchemaSprite::default();
        let start_tile = glm::vec2(1.0, 7.0);

        let ValidTile::Valid(tile_pos) = r_tile_map.get_tile_pos(start_tile) else {
            return Err(QPError::Generic(
                "[player controller] invalid start tile".into(),
            ));
        };
        let transform = CTransform2D {
            translate: tile_pos.xy(),
            scale: glm::vec2(1.0, 1.0),
            ..CTransform2D::default()
        };
        let quad = CQuad {
            width: PLAYER_SIZE,
            height: PLAYER_SIZE,
            ..CQuad::default()
        };

        this_schema.transform = transform;
        this_schema.quad = quad;
        this_schema.tag = "sprite".into();
        this_schema.texture = Some("Player.png".into());

        let id = this_schema.build_entity(registry)?;

        Ok(Self {
            player: id,
            tile: start_tile,
            tile_map,
        })
    }
}

impl Controller for PlayerController {
    fn update(&mut self, world: &mut World) -> FrameResult {
        let mut new_tile = self.tile;
        for event in world.events.iter() {
            match event {
                Event::KeyDown {
                    keycode,
                    repeat: false,
                    ..
                } => match keycode {
                    Some(Keycode::W) => new_tile.y += 1.0,
                    Some(Keycode::S) => new_tile.y -= 1.0,
                    Some(Keycode::A) => new_tile.x -= 1.0,
                    Some(Keycode::D) => new_tile.x += 1.0,
                    _ => (),
                },
                _ => (),
            };
        }

        let Some(tile_map) = world.registry.asset_manager.get::<RTileMap>(self.tile_map) else {
            return FrameResult::None;
        };
        let ValidTile::Valid(tile_val) = tile_map.get_tile_value(new_tile) else {
            return FrameResult::None;
        };
        if tile_val == 3 || tile_val == 1 {
            return FrameResult::None;
        }

        let ValidTile::Valid(tile_pos) = tile_map.get_tile_pos(new_tile) else {
            return FrameResult::None;
        };

        let new_translate = tile_pos.xy();
        if let Some(transform) = world
            .registry
            .entity_manager
            .get_mut::<CTransform2D>(&self.player)
        {
            self.tile = new_tile;
            transform.translate = new_translate;
        }

        FrameResult::None
    }
}
