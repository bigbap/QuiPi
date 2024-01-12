use engine::{
    Component,
    VersionedIndex,
    gfx::Texture,
};

#[derive(Debug, Default, Component)]
pub struct DrawComponent {
    pub shader_id: VersionedIndex,
    pub textures: Vec<(String, Texture)>
}

