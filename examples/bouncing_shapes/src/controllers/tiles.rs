use quipi_2d::{components::{sprite::TextureAtlas, CQuad, CSprite, CTransform2D}, resources::RTileMap};
use quipi_core::{
    core::canvas::get_dimensions,
    math::random::Random,
    utils::now_secs,
    FrameResponse,
    FrameState,
    IController,
    Registry,
    VersionedIndex
};

pub struct TileControler {
    _tiles: Vec<VersionedIndex>,

    pub tile_map: u64
}

impl TileControler {
    pub fn new(registry: &mut Registry) -> Result<Self, Box<dyn std::error::Error>> {
        let (_x, _y, width, height) = get_dimensions();
        let columns = width / 50;
        let rows = height / 50;
        let mut rand = Random::from_seed(now_secs()?);
        let mut tiles = vec![];
        let mut data = vec![];
        for x in 0..columns {
            for y in 0..rows {
                let tile_val = match rand.random() > 0.9 {
                    true => 0,
                    false => 1,
                };
                data.push(tile_val);
                tiles.push(tile(x as u32, y as u32, tile_val, registry));
            }
        }

        let tile_map = registry.load_resourse(
            "tile_map".to_string(),
            RTileMap::new(columns as usize, rows as usize, data, glm::vec2(32.0, 32.0))?
        )?;

        Ok(Self {
            _tiles: tiles,
            tile_map
        })
    }
}

impl IController for TileControler {
    fn update(&mut self, _frame_state: &mut FrameState, _registry: &mut Registry) -> FrameResponse {
        FrameResponse::None
    }
}

fn tile(
    x: u32,
    y: u32,
    tile_val: u16,
    registry: &mut Registry) -> VersionedIndex {
    let x_offset = (x + 0) as f32 * 32.0;
    let y_offset = (y + 0) as f32 * 32.0;

    let transform = CTransform2D {
        translate: glm::vec2(0.0 + x_offset, 0.0 + y_offset),
        scale: glm::vec2(1.0, 1.0),
        ..CTransform2D::default()
    };

    let quad = CQuad {
        center_x: x as f32,
        center_y: y as f32,
        width: 32.0,
        height: 32.0,
    };

    let entity = registry.entities.create();
    registry.entities.add(&entity, transform);
    registry.entities.add(&entity, CSprite::new(&quad, None, Some(TextureAtlas {
        texture: registry.string_interner.intern("tiles.png".to_string()),
        active_texture: match tile_val {
            0 => glm::vec2(1.0, 0.0),
            _ => glm::vec2(0.0, 0.0)
        },
        texture_dims: glm::vec2(2.0, 1.0)
    })));

    entity
}