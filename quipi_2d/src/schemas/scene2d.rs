use quipi_core::{
    components::CDrawable,
    opengl::textures::{
        ParameterName,
        ParameterValue
    },
    rendering::texture::from_image,
    resources::RTexture,
    utils::to_abs_path
};
use serde::{Serialize, Deserialize};

use crate::{
    components::{
        CScene,
        CTransform2D
    },
    Registry,
    VersionedIndex
};

use super::{
    ISchema, SchemaCamera2D, SchemaSprite, SchemaSpriteShader
};

pub const DEFAULT_SCENE: &str = "default_scene";

/**
* SCENE CONFIG
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchemaScene2D {
    pub name:       String,
    pub cameras:    Vec<SchemaCamera2D>,
    pub shaders:    Vec<SchemaSpriteShader>,
    pub textures:   Vec<String>,

    pub entities:   Vec<SchemaSprite>,
}

impl ISchema for SchemaScene2D {
    fn build_entity(
        &self,
        registry: &mut Registry
    ) -> Result<VersionedIndex, Box<dyn std::error::Error>> {
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
        for texture_name in self.textures.iter() {
            let path = format!("assets/textures/{}", texture_name);

            let texture = from_image(&to_abs_path(&path)?)?;
            texture
                .set_parameter(ParameterName::WrapS, ParameterValue::ClampToEdge)
                .set_parameter(ParameterName::WrapT, ParameterValue::ClampToEdge)
                .set_parameter(ParameterName::MinFilter, ParameterValue::LinearMipmapNearest)
                .set_parameter(ParameterName::MagFilter, ParameterValue::Nearest);

            textures.push(registry.load_resourse(texture_name.to_string(), RTexture {
                texture
            })?);
        }

        // 3. build entities
        for rect in self.entities.iter() {
            rect.build_entity(registry)?;
        }

        let entity = registry.entities.create();
        registry.entities.add(&entity, CScene {
            id: registry.string_interner.intern(self.name.clone()),
            cameras,
            shaders,
            textures
        });

        Ok(entity)
    }

    fn from_entity(entity: VersionedIndex, registry: &Registry) -> Option<Self> {
        if let Some(scene) = registry.entities.get::<CScene>(&entity) {
            // 1. new default scene schema
            let mut schema = Self {
                name: registry.string_interner.get_string(scene.id)?,
                cameras: vec![],
                shaders: vec![],
                textures: vec![],
                entities: vec![],
            };

            // 2. parse the cameras
            for id in scene.cameras.iter() {
                schema.cameras.push(SchemaCamera2D::from_resource(*id, registry)?);
            }

            // 2. parse the shaders
            for id in scene.shaders.iter() {
                schema.shaders.push(SchemaSpriteShader::from_resource(*id, registry)?);
            }

            // 3. parse textures
            for id in scene.textures.iter() {
                if registry.get_resource::<RTexture>(*id).is_some() {
                    schema.textures.push(registry.string_interner.get_string(*id)?);
                }
            }

            // 4. parse the entities
            let entities = registry.entities.query_all::<CDrawable>();
            for entity in entities {
                schema.entities.push(SchemaSprite::from_entity(entity, registry)?);
            }

            return Some(schema)
        }

        None
    }
}

impl Default for SchemaScene2D {
    fn default() -> Self {
        let shader = SchemaSpriteShader::default();

        let camera = SchemaCamera2D::default();
        let rect = SchemaSprite {
            camera: camera.name.clone(),
            transform: CTransform2D {
                translate: glm::vec2(
                    camera.right / 2.0,
                    camera.top / 2.0
                ),
                ..CTransform2D::default()
            },
            ..SchemaSprite::default()
        };

        Self {
            name: DEFAULT_SCENE.to_string(),
            cameras: vec![camera],
            shaders: vec![shader],
            textures: vec![],
            entities: vec![rect]
        }
    }
}
