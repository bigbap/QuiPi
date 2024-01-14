use engine::{
    Component,
    VersionedIndex,
    gfx::Material,
};

#[derive(Debug, Default, Component)]
pub struct DrawComponent {
    pub shader_id: VersionedIndex,
    pub camera_id: VersionedIndex,
    pub materials: Vec<Material>,
    pub color: Option<(f32, f32, f32)>
}
