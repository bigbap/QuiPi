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
        CViewMatrix
    }
};

pub fn build_perspective_camera(
    registry: &mut Registry,
    position: (f32, f32, f32),
    fov: f32,
    aspect_ratio: f32,
    near_plane: f32,
    far_plane: f32
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    registry.create_entity("camera")?
        .with(CPosition {
            x: position.0,
            y: position.1,
            z: position.2
        })?
        .with(CGizmo3D {
            front: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 1.0, 0.0)
        })?
        .with(CViewSettings {
            fov,
            aspect_ratio
        })?
        .with(CEulerAngles {
            pitch: 0.0,
            yaw: 90.0,
            roll: 0.0
        })?
        .with(CZPlanes {
            near_plane,
            far_plane
        })?
        .with(CVelocity {
            x: 0.0,
            y: 0.0,
            z: 0.0
        })?
        .with(CProjectionMatrix::default())?
        .with(CViewMatrix::default())?
        .done()
}

pub fn build_ortho_camera(
    registry: &mut Registry,
    position: (f32, f32, f32),
    width: f32,
    height: f32,
    near_plane: f32,
    far_plane: f32
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    registry.create_entity("camera")?
        .with(CPosition {
            x: position.0,
            y: position.1,
            z: position.2
        })?
        .with(CDimensions {
            width,
            height,
            ..CDimensions::default()
        })?
        .with(CGizmo3D {
            front: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 1.0, 0.0)
        })?
        .with(CEulerAngles {
            pitch: 0.0,
            yaw: 90.0,
            roll: 0.0
        })?
        .with(CZPlanes {
            near_plane,
            far_plane
        })?
        .with(CVelocity {
            x: 0.0,
            y: 0.0,
            z: 0.0
        })?
        .with(CProjectionMatrix::default())?
        .with(CViewMatrix::default())?
        .done()
}
