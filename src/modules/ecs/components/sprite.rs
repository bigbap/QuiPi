use serde::{Serialize, Deserialize};

use crate::{
    core::rendering::{
        batch::IMesh,
        vertex::Vertex
    },
    Component
};

use super::CQuad;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TextureAtlas {
    pub texture: u64,
    pub texture_dims: glm::Vec2,
    pub active_texture: glm::Vec2
}

#[derive(Debug, Component, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct CSprite {
    pub color: glm::Vec4,
    pub texture_atlas: Option<TextureAtlas>,

    mvp: glm::Mat4,
    positions: [glm::Vec4; 4],
}

impl CSprite {
    pub fn new(
        quad: &CQuad,
        color: Option<glm::Vec4>,
        texture_atlas: Option<TextureAtlas>
    ) -> Self {
        Self {
            color: match color {
                Some(c) => c,
                _ => glm::vec4(1.0, 1.0, 1.0, 1.0)
            },
            texture_atlas,
            mvp: glm::Mat4::identity(),
            positions: quad.positions()
        }
    }

    pub fn apply_matrices(
        &mut self,
        model: glm::Mat4,
        view: glm::Mat4,
        projection: glm::Mat4,
    ) {
        self.mvp = projection * view * model;
    }
}

impl IMesh for CSprite {
    fn indices() -> Vec<i32> { CQuad::indices().to_vec() }
    fn vertex_count() -> usize { 4 }

    fn vertices(&self) -> Vec<Vertex> {
        let pos1 = self.mvp * self.positions[0];
        let pos2 = self.mvp * self.positions[1];
        let pos3 = self.mvp * self.positions[2];
        let pos4 = self.mvp * self.positions[3];

        let mut x_dim = 1.0;
        let mut y_dim = 1.0;
        let mut x_offset = 0.0;
        let mut y_offset = 0.0;
        if let Some(atlas) = &self.texture_atlas {
            x_dim = atlas.texture_dims.x;
            y_dim = atlas.texture_dims.y;
            x_offset = atlas.active_texture.x / x_dim;
            y_offset = atlas.active_texture.y / y_dim;
        }

        vec![
            Vertex {
                position: pos1.xyz(),
                color: self.color,
                tex_coords: glm::vec2((1.0 / x_dim) + x_offset, (1.0 / y_dim) + y_offset),
                tex_index: 0.0
            },
            Vertex {
                position: pos2.xyz(),
                color: self.color,
                tex_coords: glm::vec2((1.0 / x_dim) + x_offset, (0.0 / y_dim) + y_offset),
                tex_index: 0.0
            },
            Vertex {
                position: pos3.xyz(),
                color: self.color,
                tex_coords: glm::vec2((0.0 / x_dim) + x_offset, (0.0 / y_dim) + y_offset),
                tex_index: 0.0
            },
            Vertex {
                position: pos4.xyz(),
                color: self.color,
                tex_coords: glm::vec2((0.0 / x_dim) + x_offset, (1.0 / y_dim) + y_offset),
                tex_index: 0.0
            }
        ]
    }
}