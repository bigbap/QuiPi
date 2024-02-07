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
    SchemaRect,
    SchemaError,
    ISchema,
    SchemaShader, rect::SchemaRectInstance,
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
    pub rects: Vec<SchemaRect>,
}

impl Default for SchemaScene {
    fn default() -> Self {
        let shader = SchemaShader::default();

        let mut camera = SchemaCamera::default();
        let rect_instance = SchemaRectInstance {
            transform: CTransform {
                translate: glm::vec3(
                    camera.params.right / 2.0,
                    camera.params.top / 2.0,
                    0.0
                ),
                ..CTransform::default()
            },
            ..SchemaRectInstance::default()
        };
        let rect = SchemaRect {
            instances: vec![rect_instance],
            ..SchemaRect::default()
        };

        camera.entities.push(rect.tag.clone());

        Self {
            tag: CTag { tag: DEFAULT_SCENE_TAG.to_string() },
            clr_color: CRGBA { r: 0.3, g: 0.3, b: 0.3, a: 1.0 },
            cameras: vec![camera],
            shaders: vec![shader],
            rects: vec![rect]
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

        // 3. build rects
        for rect in self.rects.iter() {
            rect.build(registry)?;
        }

        registry.entities.start_create()?;
        registry.entities.add(self.tag.clone());
        let entity = registry.entities.end_create()?;

        Ok(entity)
    }
}
