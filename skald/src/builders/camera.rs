use crate::{
    VersionedIndex,
    Registry,
    components::{
        CGizmo3D,
        CEulerAngles,
        CVelocity,
        CViewMatrix,
        CMouseBtnState,
        CTransform,
        CCamera, CBoundingBox,
    },
};

pub fn build_perspective_camera(
    registry: &mut Registry,
    fov: f32,
    view: CBoundingBox,
    transform: CTransform,
    angles: CEulerAngles
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    registry.create_entity("camera")?
        .with(CCamera::new_perspective(
            (view.right - view.left).abs() / (view.top - view.bottom).abs(),
            fov,
            view.near,
            view.far
        )?)?
        .with(view)?
        .with(transform)?
        .with(CGizmo3D::new(
            glm::vec3(0.0, 0.0, -1.0),
            glm::vec3(0.0, 1.0, 0.0)
        ))?
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
    frustrum: CBoundingBox,
    transform: CTransform,
    angles: CEulerAngles
) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
    registry.create_entity("camera")?
        .with(CCamera::new_orthographic(
            frustrum.left,
            frustrum.right,
            frustrum.bottom,
            frustrum.top,
            frustrum.near,
            frustrum.far
        )?)?
        .with(frustrum)?
        .with(transform)?
        .with(CGizmo3D::new(
            glm::vec3(0.0, 0.0, -1.0),
            glm::vec3(0.0, 1.0, 0.0)
        ))?
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
