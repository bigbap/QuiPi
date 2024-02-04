use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::{
    components::CRGBA,
    resources::{
        Shader,
        shader::UniformVariable
    },
    Registry, VersionedIndex,
};

use super::{SchemaCamera, SchemaRect};

/**
* SCENE CONFIG
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaScene {
    pub clr_color: CRGBA,
    pub cameras: Vec<SchemaCamera>,
    pub shaders: HashMap<String, Vec<UniformVariable>>,
    pub rects: Vec<SchemaRect>,
}

impl Default for SchemaScene {
    fn default() -> Self {
        let default_id = "default".to_string();

        let mut shaders = HashMap::<String, Vec<UniformVariable>>::new();
        shaders.insert(default_id.clone(), vec![
            UniformVariable::MVPMatrix("mvpMatrix".to_string())
        ]);

        let mut camera = SchemaCamera::default();
        camera.entities.push("rect".to_string());

        Self {
            clr_color: CRGBA { r: 0.3, g: 0.3, b: 0.3, a: 1.0 },
            cameras: vec![camera],
            shaders,
            rects: vec![SchemaRect::default()]
        }
    }
}

impl SchemaScene {
    pub fn build_scene(
        &self,
        registry: &mut Registry
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 1. build cameras
        for camera in self.cameras.iter() {
            camera.build_camera(registry)?;
        }

        // 2. build shaders
        let mut shader_map: HashMap<String, VersionedIndex> = HashMap::new();
        for (key, uniforms) in self.shaders.iter() {
            let id = registry.create_resource(
                key,
                Shader::new(key, uniforms.to_vec())?
            )?;

            shader_map.insert(key.to_string(), id);
        }

        // 3. build rects
        for rect in self.rects.iter() {
            if let Some(shader) = shader_map.get(&rect.shader) {
                rect.build_rect(
                    registry,
                    shader
                )?;
            } else {
                #[cfg(debug_assertions)]
                println!("could not find shader {}", rect.shader);
            }
        }

        Ok(())
    }
}
