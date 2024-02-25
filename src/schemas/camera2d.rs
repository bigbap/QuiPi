use serde::{Serialize, Deserialize};

use crate::QPResult;
use crate::prelude::{
    qp_ecs::components::CTransform2D,
    qp_assets::RCamera2D,
    GlobalRegistry,
    qp_data::{
        ISchema,
        OrthographicCameraParams
    }
};

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
        registry: &mut GlobalRegistry
    ) -> QPResult<u64> {

        Ok(
            registry.asset_manager.load_asset(
                self.name.clone(),
                RCamera2D::new(
                    self.params(),
                    self.transform
                )?
            )?
        )
    }

    fn from_resource(id: u64, registry: &GlobalRegistry) -> Option<Self> {
        if let Some(camera) = registry.asset_manager.get::<RCamera2D>(id) {
            let schema = Self {
                name: registry.string_interner.borrow().get_string(id)?,
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
    fn params(&self) -> OrthographicCameraParams {
        OrthographicCameraParams {
            left: self.left,
            right: self.right,
            bottom: self.bottom,
            top: self.top,
            near: self.near,
            far: self.far,
        }
    }
}
