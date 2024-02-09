use serde::{Serialize, Deserialize};

use crate::{
    components::{
        CRGBA,
        CTransform,
        CTag
    },
    Registry,
    VersionedIndex,
};

use super::{
    SchemaCamera,
    SchemaEntity2D,
    SchemaError,
    ISchema,
    SchemaShader,
};

pub const DEFAULT_SCENE_TAG: &str = "default_scene";

/**
* SCENE CONFIG
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaScene {
    pub tag: CTag,
    pub clr_color: CRGBA,
    pub cameras: Vec<SchemaCamera>,
    pub shaders: Vec<SchemaShader>,
    pub entities: Vec<SchemaEntity2D>,
}

impl Default for SchemaScene {
    fn default() -> Self {
        let shader = SchemaShader::default();

        let mut camera = SchemaCamera::default();
        let rect = SchemaEntity2D {
            transform: CTransform {
                translate: glm::vec3(
                    camera.params.right / 2.0,
                    camera.params.top / 2.0,
                    0.0
                ),
                ..CTransform::default()
            },
            ..SchemaEntity2D::default()
        };

        camera.entities.push(rect.tag.clone());

        Self {
            tag: CTag { tag: DEFAULT_SCENE_TAG.to_string() },
            clr_color: CRGBA { r: 0.3, g: 0.3, b: 0.3, a: 1.0 },
            cameras: vec![camera],
            shaders: vec![shader],
            entities: vec![rect]
        }
    }
}

impl ISchema for SchemaScene {
    fn build(
        &self,
        registry: &mut Registry
    ) -> Result<VersionedIndex, SchemaError> {
        // 1. build cameras
        for camera in self.cameras.iter() {
            camera.build(registry)?;
        }

        // 2. build shaders
        for shader in self.shaders.iter() {
            shader.build(registry)?;
        }

        // 3. build entities
        for rect in self.entities.iter() {
            rect.build(registry)?;
        }

        let entity = registry.entities.create()?;
        registry.entities.add(&entity, self.tag.clone());

        Ok(entity)
    }
}
