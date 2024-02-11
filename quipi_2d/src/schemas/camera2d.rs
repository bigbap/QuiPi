use serde::{Serialize, Deserialize};

use crate::{
    components::{
        camera::CameraParams,
        CBoundingBox2D,
        CCamera2D,
        CName,
        CTransform2D,
        CVelocity2D,
        CViewMatrix2D
    },
    Registry,
    VersionedIndex
};

use super::ISchema;

pub const DEFAULT_CAMERA: &str = "default_camera";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaCamera2D {
    pub name: CName,
    pub transform: CTransform2D,
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
            transform: CTransform2D::default(),
        }
    }
}

impl ISchema for SchemaCamera2D {
    fn build(
        &self,
        registry: &mut Registry
    ) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
        let b_box = CBoundingBox2D {
            left: self.left,
            right: self.right,
            bottom: self.bottom,
            top: self.top,
            ..CBoundingBox2D::default()
        };

        let camera = registry.entities.create()?;
        registry.entities.add(&camera, self.name.clone());
        registry.entities.add(&camera, CCamera2D::new(self.params())?);
        registry.entities.add(&camera, b_box);
        registry.entities.add(&camera, self.transform);
        registry.entities.add(&camera, CVelocity2D::default());
        registry.entities.add(&camera, CViewMatrix2D::default());

        CViewMatrix2D::update_view_matrix(&camera, registry);

        Ok(camera)
    }

    fn from_entity(entity: VersionedIndex, registry: &Registry) -> Option<Self> {
        if let (Some(camera), Some(transform), Some(name)) = (
            registry.entities.get::<CCamera2D>(&entity),
            registry.entities.get::<CTransform2D>(&entity).cloned(),
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
            left: self.left,
            right: self.right,
            bottom: self.bottom,
            top: self.top,
            near: self.near,
            far: self.far,
        }
    }
}
