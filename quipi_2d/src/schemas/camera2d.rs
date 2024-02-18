use serde::{Serialize, Deserialize};

use crate::{
    components::{
        CTransform2D,
        CVelocity2D,
    },
    resources::{
        RCamera2D,
        camera::CameraParams
    },
    Registry,
};

use super::ISchema;

pub const DEFAULT_CAMERA: &str = "default_camera";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaCamera2D {
    pub name: String,
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
            name: DEFAULT_CAMERA.to_string(),
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
    fn load_resource(
        &self,
        registry: &mut Registry
    ) -> Result<u64, Box<dyn std::error::Error>> {

        Ok(registry.load_resourse(
            self.name.clone(),
            RCamera2D::new(
                self.params(),
                self.transform,
                CVelocity2D::default()
            )?
        )?)
    }

    fn from_resource(id: u64, registry: &Registry) -> Option<Self> {
        if let Some(camera) = registry.get_resource::<RCamera2D>(id) {
            let schema = Self {
                name: registry.string_interner.get_string(id)?,
                transform: camera.transform,
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
