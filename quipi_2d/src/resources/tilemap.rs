use quipi_core::Component;
use serde::{Deserialize, Serialize};

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
    ) -> Result<Self, Box<dyn std::error::Error>> {
        println!("data.len(): {}, columns * rows: {}", data.len(), columns * rows);
        if data.len() != columns * rows {
            return Err("data doesn't match provided dimensions".into());
        }

        Ok(Self {
            data,
            columns,
            rows,
            tile_size
        })
    }
    pub fn get_tile_value(&self, (row, column): (usize, usize)) -> u16 {
        let index = (row * self.columns) + column;

        self.data[index]
    }

    pub fn get_tile_pos(&self, (row, column): (usize, usize)) -> glm::Vec3 {
        let pos_x = column as f32 * self.tile_size.x;
        let pos_y = row as f32 * self.tile_size.y;

        glm::vec3(pos_x, pos_y, 0.0)
    }
}