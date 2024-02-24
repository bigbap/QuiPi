use serde::{Serialize, Deserialize};

use super::super::prelude::Component;

#[derive(Debug, Component, Serialize, Deserialize, Clone, PartialEq)]
pub struct CQuad {
    pub width: f32,
    pub height: f32,
    pub center_x: f32,
    pub center_y: f32,
}

impl CQuad {
    pub fn indices() -> [i32; 6] { [0, 1, 3, 1, 2, 3] }
    pub fn positions(&self) -> [glm::Vec4; 4] {
        let pos1 = glm::vec4(self.center_x + (self.width / 2.0), self.center_y + (self.height / 2.0), 0.0, 1.0);
        let pos2 = glm::vec4(self.center_x + (self.width / 2.0), self.center_y - (self.height / 2.0), 0.0, 1.0);
        let pos3 = glm::vec4(self.center_x - (self.width / 2.0), self.center_y - (self.height / 2.0), 0.0, 1.0);
        let pos4 = glm::vec4(self.center_x - (self.width / 2.0), self.center_y + (self.height / 2.0), 0.0, 1.0);

        [
            pos1, // top right
            pos2, // bottom right
            pos3, // bottom left
            pos4, // top left
        ]
    }
}

impl Default for CQuad {
    fn default() -> Self {
        Self {
            center_x: 0.0,
            center_y: 0.0,
            width: 200.0,
            height: 200.0,
        }
    }
}
