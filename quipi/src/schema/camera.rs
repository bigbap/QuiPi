
use serde::{Serialize, Deserialize};

use crate::{
    components::{
        CTransform,
        CCamera,
        CBoundingBox,
        CGizmo3D,
        CVelocity, CViewMatrix, CPrefab
    },
    VersionedIndex,
    Registry
};

use super::{ISchema, SchemaError};

pub const DEFAULT_CAMERA_TAG: &str = "default_camera";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaCamera {
    pub tag: String,
    pub params: CameraParams,
    pub transform: CTransform,
    pub entities: Vec<String>
}

impl Default for SchemaCamera {
    fn default() -> Self {
        Self {
            tag: DEFAULT_CAMERA_TAG.to_string(),
            params: CameraParams::default(),
            transform: CTransform::default(),
            entities: vec![],
        }
    }
}

impl ISchema for SchemaCamera {
    fn build(
        &self,
        registry: &mut Registry
    ) -> Result<VersionedIndex, SchemaError> {
        let camera = registry.create_entity(&self.tag)?
            .with(CCamera::new(self.params)?)?
            .with(CBoundingBox {
                left: self.params.left,
                right: self.params.right,
                bottom: self.params.bottom,
                top: self.params.top,
                ..CBoundingBox::default()
            })?
            .with(CGizmo3D::default())?
            .with(self.transform)?
            .with(CVelocity::default())?
            .with(CViewMatrix::default())?
            .with(CPrefab { schema: Box::new(self.to_owned()) })?
            .done()?;

        CViewMatrix::update_view_matrix(&camera, registry);

        Ok(camera)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum CameraKind {
    Cam3D,
    Cam2D
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct CameraParams {
    pub kind: CameraKind,
    pub fov: f32,
    pub aspect: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32
}

impl Default for CameraParams {
    fn default() -> Self {
        Self {
            kind: CameraKind::Cam2D,
            fov: 0.0,
            aspect: 0.0,
            left: 0.0,
            right: 800.0,
            bottom: 0.0,
            top: 600.0,
            near: 0.0,
            far: 0.2
        }
    }
}

