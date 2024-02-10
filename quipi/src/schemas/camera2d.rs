
use serde::{Serialize, Deserialize};

use crate::{
    components::{
        camera::{
            CameraKind,
            CameraParams
        },
        CBoundingBox,
        CCamera,
        CName,
        CTransform,
        CVelocity,
        CViewMatrix
    },
    Registry,
    VersionedIndex
};

use super::{ISchema, SchemaError};

pub const DEFAULT_CAMERA: &str = "default_camera";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaCamera2D {
    pub name: CName,
    pub transform: CTransform,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for SchemaCamera2D {
    fn default() -> Self {
        Self {
            name: CName { name: DEFAULT_CAMERA.to_string() },
            left: 0.0,
            right: 800.0,
            bottom: 0.0,
            top: 600.0,
            near: 0.0,
            far: 0.2,
            transform: CTransform::default(),
        }
    }
}

impl ISchema for SchemaCamera2D {
    fn build(
        &self,
        registry: &mut Registry
    ) -> Result<VersionedIndex, SchemaError> {
        let b_box = CBoundingBox {
            left: self.left,
            right: self.right,
            bottom: self.bottom,
            top: self.top,
            ..CBoundingBox::default()
        };

        let camera = registry.entities.create()?;
        registry.entities.add(&camera, self.name.clone());
        registry.entities.add(&camera, CCamera::new(self.params())?);
        registry.entities.add(&camera, b_box);
        registry.entities.add(&camera, self.transform);
        registry.entities.add(&camera, CVelocity::default());
        registry.entities.add(&camera, CViewMatrix::default());

        CViewMatrix::update_view_matrix(&camera, registry);

        Ok(camera)
    }

    fn from_entity(entity: VersionedIndex, registry: &Registry) -> Option<Self> {
        if let (Some(camera), Some(transform), Some(name)) = (
            registry.entities.get::<CCamera>(&entity),
            registry.entities.get::<CTransform>(&entity).cloned(),
            registry.entities.get::<CName>(&entity).cloned(),
        ) {
            let schema = Self {
                name,
                transform,
                left: camera.params.left,
                right: camera.params.right,
                bottom: camera.params.bottom,
                top: camera.params.top,
                near: camera.params.near,
                far: camera.params.far,
            };

            return Some(schema)
        }

        None
    }
}

impl SchemaCamera2D {
    fn params(&self) -> CameraParams {
        CameraParams {
            kind: CameraKind::Cam2D,
            left: self.left,
            right: self.right,
            bottom: self.bottom,
            top: self.top,
            near: self.near,
            far: self.far,
            ..CameraParams::default()
        }
    }
}
