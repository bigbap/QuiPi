use serde::{Deserialize, Serialize};

use crate::prelude::{
    qp_ecs::{
        components::{CScene, CSprite, CTransform2D},
        Index,
    },
    GlobalRegistry, Schema,
};
use crate::QPResult;

use super::prelude::{SchemaCamera2D, SchemaShader, SchemaSprite, SchemaTexture};

pub const DEFAULT_SCENE: &str = "default_scene";

/**
* SCENE CONFIG
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaScene2D {
    pub name: String,
    pub cameras: Vec<SchemaCamera2D>,
    pub shaders: Vec<SchemaShader>,
    pub textures: Vec<SchemaTexture>,

    pub sprites: Vec<SchemaSprite>,
}

impl Schema for SchemaScene2D {
    fn build_entity(&self, registry: &mut GlobalRegistry) -> QPResult<Index> {
        // 1. build cameras
        let mut cameras = vec![];
        for camera in self.cameras.iter() {
            cameras.push(camera.load_resource(registry)?);
        }

        // 2. build shaders
        let mut shaders = vec![];
        for shader in self.shaders.iter() {
            shaders.push(shader.load_resource(registry)?);
        }

        // 2. build textures
        let mut textures = vec![];
        for texture in self.textures.iter() {
            textures.push(texture.load_resource(registry)?);
        }

        // 3. build entities
        for rect in self.sprites.iter() {
            rect.build_entity(registry)?;
        }

        let id = registry.strings_mut().intern(self.name.clone());

        let entity = registry.entities.create(CScene {
            id,
            cameras,
            shaders,
            textures,
        });

        Ok(entity)
    }

    fn from_entity(entity: Index, registry: &GlobalRegistry) -> Option<Self> {
        if let Some(scene) = registry.entities.get::<CScene>(&entity) {
            // 1. new default scene schema
            let mut schema = Self {
                name: registry.strings().get_string(scene.id)?,
                cameras: vec![],
                shaders: vec![],
                textures: vec![],
                sprites: vec![],
            };

            // 2. parse the cameras
            for id in scene.cameras.iter() {
                schema
                    .cameras
                    .push(SchemaCamera2D::from_resource(*id, registry)?);
            }

            // 2. parse the shaders
            for id in scene.shaders.iter() {
                schema
                    .shaders
                    .push(SchemaShader::from_resource(*id, registry)?);
            }

            // 3. parse textures
            for id in scene.textures.iter() {
                schema
                    .textures
                    .push(SchemaTexture::from_resource(*id, registry)?);
            }

            // 4. parse the entities
            let entities = registry.entities.query_all::<CSprite>();
            for entity in entities {
                schema
                    .sprites
                    .push(SchemaSprite::from_entity(entity, registry)?);
            }

            return Some(schema);
        }

        None
    }
}

impl Default for SchemaScene2D {
    fn default() -> Self {
        let shader = SchemaShader::default();

        let camera = SchemaCamera2D::default();
        let sprite = SchemaSprite {
            transform: CTransform2D {
                translate: glm::vec2(camera.right / 2.0, camera.top / 2.0),
                ..CTransform2D::default()
            },
            ..SchemaSprite::default()
        };

        Self {
            name: DEFAULT_SCENE.to_string(),
            cameras: vec![camera],
            shaders: vec![shader],
            textures: vec![],
            sprites: vec![sprite],
        }
    }
}
