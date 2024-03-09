mod sprite;
// mod text;

pub use sprite::SpriteRenderer;

use crate::registry::GlobalRegistry;
// pub use text::*;

pub trait Renderer {
    fn draw(&mut self, registry: &mut GlobalRegistry) -> Option<u32>;
}
