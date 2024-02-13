pub mod canvas;
pub mod mesh;
pub mod texture;

use crate::{
    components::CTag,
    Registry,
    VersionedIndex
};

#[derive(Debug, Default)]
pub struct RenderInfo {
    pub num_draw_calls: u32,
    pub total_ms: f32
}

pub trait IRenderer {
    fn batch_render(&mut self, tag: CTag, registry: &mut Registry) -> Result<(), Box<dyn std::error::Error>>;
    fn instance_render(&mut self, tag: CTag, registry: &mut Registry) -> Result<(), Box<dyn std::error::Error>>;
    fn single_render(&mut self, entity: VersionedIndex, registry: &mut Registry) -> Result<(), Box<dyn std::error::Error>>;

    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn flush(&mut self, registry: &Registry) -> RenderInfo;
}