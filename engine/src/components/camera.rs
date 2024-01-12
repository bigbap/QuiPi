use crate::ecs::{
    Component,
    VersionedIndex
};

#[derive(Debug, Component)]
pub struct CameraComponent {
    pub id: VersionedIndex
}
