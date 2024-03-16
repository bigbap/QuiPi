use crate::prelude::qp_common::components::*;
use crate::prelude::*;
use serde::{Deserialize, Serialize};

const PERSPECTIVE_NEAR: f32 = 0.1;
const PERSPECTIVE_FAR: f32 = 100.0;
const ORTHOGRAPHIC_NEAR: f32 = 0.1;
const ORTHOGRAPHIC_FAR: f32 = 2.0;

#[derive(Debug, Default, Component, Serialize, Deserialize, PartialEq, Clone)]
pub enum CCameraKind {
    #[default]
    Orthographic,
    Perspective(f32),
}

#[derive(Debug, Component, Serialize, Deserialize, PartialEq, Clone)]
pub struct CRenderLayer(Vec<u32>);
impl Default for CRenderLayer {
    fn default() -> Self {
        Self(vec![0])
    }
}

#[derive(Default)]
pub struct CameraMetadata {
    pub kind: CCameraKind,
    pub width: u32,
    pub height: u32,
    pub render_layer: CRenderLayer,
}

#[derive(Debug, Component, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct CCamera {}

#[derive(Debug, Component, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct CMatrix4(pub glm::Mat4);

pub fn camera_bundle(metadata: CameraMetadata) -> impl Bundle {
    let mut camera_bundle = BundleBuilder::default();
    let transform = CTransform::default();
    let position = glm::vec3(transform.translate.x, transform.translate.y, 0.0);

    camera_bundle.add_bundle((
        CCamera::default(),
        metadata.kind.clone(),
        CTransform::default(),
        metadata.render_layer,
    ));

    match &metadata.kind {
        CCameraKind::Orthographic => camera_bundle.add_bundle((
            COrthographic {
                left: 0.0,
                right: metadata.width as f32,
                bottom: 0.0,
                top: metadata.height as f32,
                near: ORTHOGRAPHIC_NEAR,
                far: ORTHOGRAPHIC_FAR,
            },
            CMatrix4(matrix_ortho(metadata.width, metadata.height, &position)),
        )),
        CCameraKind::Perspective(fov) => camera_bundle.add_bundle((
            CPerspective {
                aspect: metadata.width as f32 / metadata.height as f32,
                fov: *fov,
                near: PERSPECTIVE_NEAR,
                far: PERSPECTIVE_FAR,
            },
            CMatrix4(matrix_perspective(
                metadata.width as f32 / metadata.height as f32,
                *fov,
                &glm::vec3(0.0, 0.0, -1.0),
                &glm::vec3(0.0, 1.0, 0.0),
                &position,
            )),
        )),
    };

    camera_bundle
}

pub fn matrix_ortho(width: u32, height: u32, position: &glm::Vec3) -> glm::Mat4 {
    glm::ortho(
        0.0,
        width as f32,
        0.0,
        height as f32,
        ORTHOGRAPHIC_NEAR,
        ORTHOGRAPHIC_FAR,
    ) * glm::look_at(
        position,
        &(position + glm::vec3(0.0, 0.0, -1.0)),
        &glm::vec3(0.0, 1.0, 0.0),
    )
}

pub fn matrix_perspective(
    aspect: f32,
    fov: f32,
    front: &glm::Vec3,
    up: &glm::Vec3,
    position: &glm::Vec3,
) -> glm::Mat4 {
    glm::perspective(aspect, fov, PERSPECTIVE_NEAR, PERSPECTIVE_FAR)
        * glm::look_at(position, &(position + front), up)
}
