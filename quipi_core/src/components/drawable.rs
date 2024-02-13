use crate::{
    Component,
    VersionedIndex,
    opengl::draw::DrawMode
};

#[derive(Debug, Component, PartialEq)]
pub struct CDrawable {
    pub shader: VersionedIndex,
    pub camera: VersionedIndex,
    pub draw_mode: DrawMode,
    pub should_draw: bool
}
