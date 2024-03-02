use quipi::{
    app::{Controller, FrameResult},
    prelude::QPError,
    schemas::sprite::TextureAtlas,
    world::World,
};

use crate::{
    qp_assets::RTileMap,
    qp_core::{now_secs, random::Random},
    qp_ecs::components::{CQuad, CSprite, CTransform2D},
    GlobalRegistry, VersionedIndex,
};

const TILE_SIZE: f32 = 64.0;
const TILE_MAP: [[u16; 10]; 10] = [
    [9, 9, 9, 9, 9, 3, 3, 3, 3, 3],
    [9, 9, 9, 9, 9, 3, 0, 0, 1, 3],
    [9, 9, 9, 9, 9, 3, 0, 1, 0, 3],
    [9, 9, 9, 9, 9, 3, 0, 1, 0, 3],
    [9, 9, 9, 9, 9, 3, 0, 0, 0, 3],
    [3, 3, 3, 3, 3, 3, 0, 0, 0, 3],
    [3, 0, 0, 0, 0, 0, 0, 0, 0, 3],
    [3, 0, 0, 1, 1, 0, 0, 1, 0, 3],
    [3, 2, 0, 0, 0, 0, 0, 0, 0, 3],
    [3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
];

pub struct TileControler {
    _tiles: Vec<VersionedIndex>,

    pub tile_map: u64,
}

impl TileControler {
    pub fn new(registry: &mut GlobalRegistry) -> Result<Self, QPError> {
        let mut _rand = Random::from_seed(now_secs()?);
        let columns = 10; // rand.range(10, 30);
        let rows = 10; // rand.range(8, 20);
        let mut tiles = vec![];
        let mut data = vec![];
        for x in 0..columns {
            for y in 0..rows {
                let tile_val = TILE_MAP[x][y];
                // let tile_val = if x == 0 || x == (columns - 1) || y == 0 || y == (rows - 1) { 3 } else {
                //     choose(&mut rand)
                // };
                data.push(tile_val);
                tiles.push(tile(x as u32, y as u32, tile_val, registry));
            }
        }

        let tile_map = registry.asset_manager.load_asset(
            "tile_map".to_string(),
            RTileMap::new(
                columns as usize,
                rows as usize,
                data,
                glm::vec2(TILE_SIZE, TILE_SIZE),
            )?,
        )?;

        Ok(Self {
            _tiles: tiles,
            tile_map,
        })
    }
}

impl Controller for TileControler {
    fn update(&mut self, _world: &mut World) -> FrameResult {
        FrameResult::None
    }
}

fn tile(x: u32, y: u32, tile_val: u16, registry: &mut GlobalRegistry) -> VersionedIndex {
    let x_offset = (x + 0) as f32 * TILE_SIZE;
    let y_offset = (y + 0) as f32 * TILE_SIZE;

    let transform = CTransform2D {
        translate: glm::vec2(0.0 + x_offset, 0.0 + y_offset),
        scale: glm::vec2(1.0, 1.0),
        ..CTransform2D::default()
    };

    let quad = CQuad {
        center_x: 0.0,
        center_y: 0.0,
        width: TILE_SIZE,
        height: TILE_SIZE,
    };

    let entity = registry.entity_manager.create();
    registry.entity_manager.add(&entity, transform);
    registry.entity_manager.add(
        &entity,
        CSprite::new(
            &quad,
            match tile_val {
                9 => Some(glm::vec4(0.0, 0.0, 0.0, 0.0)),
                _ => None,
            },
            match tile_val {
                9 => None,
                _ => Some(TextureAtlas {
                    texture: registry.strings_mut().intern("tiles.png".to_string()),
                    active_texture: glm::vec2(tile_val as f32, 0.0),
                    texture_dims: glm::vec2(4.0, 1.0),
                }),
            },
        ),
    );

    entity
}

fn _choose(rand: &mut Random) -> u16 {
    let n = rand.random();

    if n < 0.7 {
        return 0;
    }

    if n < 0.95 {
        return 1;
    }

    2
}
