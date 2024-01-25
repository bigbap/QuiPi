use crate::{
    VersionedIndex,
    Registry,
    components::{
        CPosition,
        CGizmo3D,
        CViewSettings,
        CEulerAngles,
        CZPlanes,
        CVelocity,
        CDimensions,
        CProjectionMatrix,
        CViewMatrix,
        CMouseBtnState,
    },
};

pub fn build_perspective_camera(
    registry: &mut Registry,
    position: (f32, f32, f32),
    fov: f32,
    aspect_ratio: f32,
    near_plane: f32,
    far_plane: f32,
    angles: CEulerAngles
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    registry.create_entity("camera")?
        .with(CPosition {
            x: position.0,
            y: position.1,
            z: position.2
        })?
        .with(CGizmo3D::new(
            glm::vec3(0.0, 0.0, -1.0),
            glm::vec3(0.0, 1.0, 0.0)
        ))?
        .with(CViewSettings {
            fov,
            aspect_ratio
        })?
        .with(angles)?
        .with(CZPlanes {
            near_plane,
            far_plane
        })?
        .with(CVelocity {
            x: 0.0,
            y: 0.0,
            z: 0.0
        })?
        .with(CMouseBtnState::default())?
        .with(CProjectionMatrix::default())?
        .with(CViewMatrix::default())?
        .done()
}

pub fn build_ortho_camera(
    registry: &mut Registry,
    width: f32,
    height: f32,
    position: CPosition,
    planes: CZPlanes,
    angles: CEulerAngles
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    registry.create_entity("camera")?
        .with(position)?
        .with(CDimensions {
            width,
            height,
            ..CDimensions::default()
        })?
        .with(CGizmo3D::new(
            glm::vec3(0.0, 0.0, -1.0),
            glm::vec3(0.0, 1.0, 0.0)
        ))?
        .with(angles)?
        .with(planes)?
        .with(CVelocity {
            x: 0.0,
            y: 0.0,
            z: 0.0
        })?
        .with(CMouseBtnState::default())?
        .with(CProjectionMatrix::default())?
        .with(CViewMatrix::default())?
        .done()
}
