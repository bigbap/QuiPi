use quipi_2d::components::{CQuad, CSprite, CTransform2D};
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
}

impl TileControler {
    pub fn new(registry: &mut Registry) -> Result<Self, Box<dyn std::error::Error>> {
        let (_x, _y, width, height) = get_dimensions();
        let mut rand = Random::from_seed(now_secs()?);
        let mut tiles = vec![];
        for x in 0..(width / 50) {
            for y in 0..(height / 50) {
                tiles.push(tile(x as u32, y as u32, &mut rand, registry));
            }
        }

        Ok(Self {
            _tiles: tiles,
        })
    }
}

impl IController for TileControler {
    fn update(&mut self, _frame_state: &mut FrameState, _registry: &mut Registry) -> FrameResponse {
        FrameResponse::None
    }
}

fn tile(x: u32, y: u32, rand: &mut Random, registry: &mut Registry) -> VersionedIndex {
    let x_offset = (x + 1) as f32 * 1.0;
    let y_offset = (y + 1) as f32 * 1.0;
    let color = match rand.random() > 0.9 {
        true => glm::vec4(0.3, 0.3, 0.3, 1.0),
        false => glm::vec4(0.6, 0.8, 0.0, 1.0),
    };

    let transform = CTransform2D {
        translate: glm::vec2(15.0 + x_offset, 15.0 + y_offset),
        scale: glm::vec2(50.0, 50.0),
        ..CTransform2D::default()
    };

    let quad = CQuad {
        center_x: x as f32,
        center_y: y as f32,
        width: 1.0,
        height: 1.0,
    };

    let entity = registry.entities.create();
    registry.entities.add(&entity, transform);
    registry.entities.add(&entity, CSprite::new(&quad, Some(color), None));

    entity
}