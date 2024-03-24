use crate::{
    assets::{Asset, AssetLoader, Source},
    platform::opengl::textures::GlTexture,
    prelude::{qp_gfx::texture, QPError},
    QPResult,
};

#[derive(Debug)]
pub struct TextureCoords {
    pub top_left: glm::Vec2,
    pub top_right: glm::Vec2,
    pub bottom_right: glm::Vec2,
    pub bottom_left: glm::Vec2,
}

impl Default for TextureCoords {
    fn default() -> Self {
        Self {
            top_left: glm::vec2(0.0, 1.0),
            top_right: glm::vec2(1.0, 1.0),
            bottom_right: glm::vec2(1.0, 0.0),
            bottom_left: glm::vec2(0.0, 0.0),
        }
    }
}

#[derive(Asset, Debug, Clone, PartialEq, Default)]
pub struct Texture {
    pub texture: GlTexture,
    pub dims: Option<(u32, u32)>,
}

impl Texture {
    pub fn get_coords_at_loc(&self, loc: (u32, u32)) -> TextureCoords {
        let Some(dims) = self.dims else {
            return TextureCoords::default();
        };

        let (x_dim, y_dim) = dims;
        let (x_offset, y_offset) = (loc.0 as f32 / x_dim as f32, loc.1 as f32 / y_dim as f32);

        TextureCoords {
            top_left: glm::vec2(
                (0.0 / x_dim as f32) + x_offset,
                (1.0 / y_dim as f32) + y_offset,
            ),
            top_right: glm::vec2(
                (1.0 / x_dim as f32) + x_offset,
                (1.0 / y_dim as f32) + y_offset,
            ),
            bottom_right: glm::vec2(
                (1.0 / x_dim as f32) + x_offset,
                (0.0 / y_dim as f32) + y_offset,
            ),
            bottom_left: glm::vec2(
                (0.0 / x_dim as f32) + x_offset,
                (0.0 / y_dim as f32) + y_offset,
            ),
        }
    }

    pub fn texture_id(&self) -> u32 {
        self.texture.id
    }
}

pub struct TextureLoader {
    pub source: Source,
    pub dims: Option<(u32, u32)>,
}

impl AssetLoader for TextureLoader {
    type AssetType = Texture;

    fn load(&mut self) -> QPResult<Texture> {
        let texture = match &self.source {
            Source::Buffer(metadata) => texture::from_buffer(
                metadata.format,
                metadata.width,
                metadata.height,
                &metadata.buffer,
            ),
            Source::Path(path) => texture::from_image(path)?,
            _ => return Err(QPError::Generic("invalid source for texture".into())),
        };

        Ok(Texture {
            texture,
            dims: self.dims,
        })
    }
}
