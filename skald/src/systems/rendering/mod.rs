use crate::{
    Registry,
    VersionedIndex
};

pub mod renderer;
pub mod renderer_2d;

pub use renderer::Renderer;
pub use renderer_2d::Renderer2D;

mod draw;
mod matrices;

pub trait IRenderer {
    fn camera(&self) -> VersionedIndex;
    fn update_view_matrix(&self, registry: &mut Registry);
}