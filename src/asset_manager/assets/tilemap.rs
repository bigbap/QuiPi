use serde::{Deserialize, Serialize};
use crate::{
    prelude::qp_data::ValidTile,
    prelude::qp_ecs::Component,
    QPResult
};

#[derive(Debug, Component, PartialEq, Clone, Serialize, Deserialize)]
pub struct RTileMap {
    pub data: Vec<u16>,
    pub columns: usize,
    pub rows: usize,
    pub tile_size: glm::Vec2,
}

impl RTileMap {
    pub fn new(
        columns: usize,
        rows: usize,
        data: Vec<u16>,
        tile_size: glm::Vec2
    ) -> QPResult<Self> {
        assert!(data.len() == columns * rows);

        Ok(Self {
            data,
            columns,
            rows,
            tile_size
        })
    }
    pub fn get_tile_value(&self, tile: glm::Vec2) -> ValidTile<u16> {
        if self.is_valid(&tile) {
            ValidTile::Valid(
                self.data[(tile.x as usize * self.rows) + tile.y as usize]
            )
        } else {
            ValidTile::Invalid
        }
    }

    pub fn get_tile_pos(&self, tile: glm::Vec2) -> ValidTile<glm::Vec3> {
        if self.is_valid(&tile) {
            let pos_x = tile.x as usize as f32 * self.tile_size.x;
            let pos_y = tile.y as usize as f32 * self.tile_size.y;

            ValidTile::Valid(glm::vec3(pos_x, pos_y, 0.0))
        } else {
            ValidTile::Invalid
        }
    }

    pub fn is_valid(&self, tile: &glm::Vec2) -> bool {
        tile.x < self.columns as f32 &&
        tile.x >= 0.0 &&
        tile.y < self.rows as f32 &&
        tile.y >= 0.0
    }
}
