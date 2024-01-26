use crate::{
    VersionedIndex,
    Registry,
    components::{
        CGizmo3D,
        CViewSettings,
        CEulerAngles,
        CVelocity,
        CDimensions,
        CViewMatrix,
        CMouseBtnState,
        CTransform,
        CCamera, CZPlanes,
    },
};

pub fn build_perspective_camera(
    registry: &mut Registry,
    fov: f32,
    aspect_ratio: f32,
    z_planes: CZPlanes,
    transform: CTransform,
    angles: CEulerAngles
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    registry.create_entity("camera")?
        .with(CCamera::new_perspective(
            aspect_ratio,
            fov,
            z_planes.near_plane,
            z_planes.far_plane
        )?)?
        .with(transform)?
        .with(CGizmo3D::new(
            glm::vec3(0.0, 0.0, -1.0),
            glm::vec3(0.0, 1.0, 0.0)
        ))?
        .with(CViewSettings {
            fov,
            aspect_ratio
        })?
        .with(z_planes)?
        .with(angles)?
        .with(CVelocity {
            x: 0.0,
            y: 0.0,
            z: 0.0
        })?
        .with(CMouseBtnState::default())?
        .with(CViewMatrix::default())?
        .done()
}

pub fn build_ortho_camera(
    registry: &mut Registry,
    width: f32,
    height: f32,
    z_planes: CZPlanes,
    transform: CTransform,
    angles: CEulerAngles
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    registry.create_entity("camera")?
        .with(CCamera::new_orthographic(
            0.0,
            width,
            0.0,
            height,
            // -(width * 0.5),
            // width * 0.5,
            // -(height * 0.5),
            // height * 0.5,
            z_planes.near_plane,
            z_planes.far_plane
        )?)?
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
        .with(z_planes)?
        .with(angles)?
        .with(CVelocity {
            x: 0.0,
            y: 0.0,
            z: 0.0
        })?
        .with(CMouseBtnState::default())?
        .with(CViewMatrix::default())?
        .done()
}
