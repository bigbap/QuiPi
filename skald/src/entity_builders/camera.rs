use crate::{
    VersionedIndex,
    Registry,
    components::{
        CGizmo3D,
        CViewSettings,
        CEulerAngles,
        CZPlanes,
        CVelocity,
        CDimensions,
        CProjectionMatrix,
        CViewMatrix,
        CMouseBtnState,
        CTransform,
    },
};

pub fn build_perspective_camera(
    registry: &mut Registry,
    fov: f32,
    aspect_ratio: f32,
    planes: CZPlanes,
    transform: CTransform,
    angles: CEulerAngles
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    registry.create_entity("camera")?
        .with(transform)?
        .with(CGizmo3D::new(
            glm::vec3(0.0, 0.0, -1.0),
            glm::vec3(0.0, 1.0, 0.0)
        ))?
        .with(CViewSettings {
            fov,
            aspect_ratio
        })?
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

pub fn build_ortho_camera(
    registry: &mut Registry,
    width: f32,
    height: f32,
    transform: CTransform,
    planes: CZPlanes,
    angles: CEulerAngles
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    registry.create_entity("camera")?
        .with(transform)?
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
