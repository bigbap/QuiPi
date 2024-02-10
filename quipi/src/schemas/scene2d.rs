use serde::{Serialize, Deserialize};

use crate::{
    components::{
        CCamera,
        CDrawable,
        CScene,
        CTransform,
        CRGBA
    },
    resources::RShader,
    Registry,
    VersionedIndex
};

use super::{
    SchemaCamera2D,
    SchemaEntity2D,
    SchemaError,
    ISchema,
    SchemaShader,
};

pub const DEFAULT_SCENE: &str = "default_scene";

/**
* SCENE CONFIG
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaScene2D {
    pub name:       CScene,
    pub clr_color:  CRGBA,
    pub cameras:    Vec<SchemaCamera2D>,
    pub shaders:    Vec<SchemaShader>,
    pub entities:   Vec<SchemaEntity2D>,
}

impl ISchema for SchemaScene2D {
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
        registry.entities.add(&entity, self.name.clone());
        registry.entities.add(&entity, self.clr_color);

        Ok(entity)
    }

    fn from_entity(entity: VersionedIndex, registry: &Registry) -> Option<Self> {
        if let (Some(name), Some(color)) = (
            registry.entities.get::<CScene>(&entity),
            registry.entities.get::<CRGBA>(&entity)
        ) {
            // 1. new default scene schema
            let mut schema = Self {
                name: name.clone(),
                clr_color: color.clone(),
                cameras: vec![],
                shaders: vec![],
                entities: vec![],
            };

            // 2. parse the cameras
            let cameras = registry.entities.query_all::<CCamera>();
            for camera in cameras {
                schema.cameras.push(SchemaCamera2D::from_entity(camera, registry)?);
            }

            // 2. parse the shaders
            let shaders = registry.resources.query_all::<RShader>();
            for shader in shaders {
                schema.shaders.push(SchemaShader::from_entity(shader, registry)?);
            }

            // 2. parse the entities
            let entities = registry.entities.query_all::<CDrawable>();
            for entity in entities {
                schema.entities.push(SchemaEntity2D::from_entity(entity, registry)?);
            }

            return Some(schema)
        }

        None
    }
}

impl Default for SchemaScene2D {
    fn default() -> Self {
        let shader = SchemaShader::default();

        let camera = SchemaCamera2D::default();
        let rect = SchemaEntity2D {
            camera: camera.name.clone(),
            transform: CTransform {
                translate: glm::vec3(
                    camera.right / 2.0,
                    camera.top / 2.0,
                    0.0
                ),
                ..CTransform::default()
            },
            ..SchemaEntity2D::default()
        };

        Self {
            name: CScene { name: DEFAULT_SCENE.to_string() },
            clr_color: CRGBA { r: 0.3, g: 0.3, b: 0.3, a: 1.0 },
            cameras: vec![camera],
            shaders: vec![shader],
            entities: vec![rect]
        }
    }
}
