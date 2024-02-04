use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::{
    components::{CRGBA, CPrefab},
    resources::{
        Shader,
        shader::UniformVariable
    },
    Registry,
    VersionedIndex,
};

use super::{
    SchemaCamera,
    SchemaRect,
    SchemaError,
    ISchema
};

pub const DEFAULT_SCENE_TAG: &str = "default_scene";
pub const DEFAULT_SHADER: &str = "default";
pub const DEFAULT_SHADER_UNIFORM: &str = "mvpMatrix";

/**
* SCENE CONFIG
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaScene {
    pub tag: String,
    pub clr_color: CRGBA,
    pub cameras: Vec<SchemaCamera>,
    pub shaders: HashMap<String, Vec<UniformVariable>>,
    pub rects: Vec<SchemaRect>,
}

impl Default for SchemaScene {
    fn default() -> Self {
        let mut shaders = HashMap::<String, Vec<UniformVariable>>::new();
        shaders.insert(DEFAULT_SHADER.to_string(), vec![
            UniformVariable::MVPMatrix(DEFAULT_SHADER_UNIFORM.to_string())
        ]);

        let mut camera = SchemaCamera::default();
        let mut rect = SchemaRect::default();
        rect.transform.translate = glm::vec3(
            camera.params.right / 2.0,
            camera.params.top / 2.0,
            0.0
        );

        camera.entities.push(rect.tag.clone());

        Self {
            tag: DEFAULT_SCENE_TAG.to_string(),
            clr_color: CRGBA { r: 0.3, g: 0.3, b: 0.3, a: 1.0 },
            cameras: vec![camera],
            shaders,
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
        for (key, uniforms) in self.shaders.iter() {
            registry.create_resource(
                key,
                Shader::new(key, uniforms.to_vec())?
            )?;
        }

        // 3. build rects
        for rect in self.rects.iter() {
            rect.build(registry)?;
        }

        Ok(registry.create_entity(&self.tag)?
            .with(CPrefab { schema: Box::new(self.to_owned()) })?
            .done()?)
    }
}
